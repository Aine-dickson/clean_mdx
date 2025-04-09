use std::iter::Peekable;

use block_token::{block_tokenization, BlockToken, CodeBlock, HrToken, ListMeta, ListToken, ListType};
use inline_token::{inline_tokenization, InlineToken, InlineTokenPos};

pub mod block_token;
pub mod inline_token;

#[derive(Debug, Clone)]
pub struct SecList {
    r#type: ListType,
    items: Vec<(ListMeta, Vec<InlineToken>)>,
    is_nested: usize,
    nests: Vec<SecList>
}

#[derive(Debug, Clone)]
/// A wrapper type for the **Inline** tokens corresponding to the givens ```BlockToken```
pub enum SecondaryToken {
    Br, Hr(HrToken),
    P, List(SecList),
    CodeBlock(CodeBlock),
    Text(Vec<InlineToken>),
    Form(Vec<InlineToken>),
    Table(Vec<InlineToken>),
    Blockquote(Vec<InlineToken>),
    Heading(usize, Vec<InlineToken>),
}

pub enum Action {
    Break,
    Continue(bool)
}

pub fn tokenizer(input: &str)-> Vec<SecondaryToken>{
    let input_lines: Vec<&str> = input.lines().collect();

    let mut block_tokens: Vec<BlockToken> = Vec::new();
    let mut secondary_tokens: Vec<SecondaryToken> = Vec::new();
    
    block_tokenization(&mut block_tokens, &input_lines);

    let mut token_iter = block_tokens.iter().peekable();

    while let Some(token) = token_iter.peek() {
        match *token {
            BlockToken::Br => secondary_tokens.push(SecondaryToken::Br),
            BlockToken::P => {
                // TODO: Implement Paragrahping
            }
            BlockToken::Hr(hr_token) => secondary_tokens.push(SecondaryToken::Hr(hr_token.clone())),
            BlockToken::Blockquote(content) => secondary_tokens.push(SecondaryToken::Blockquote(inline_tokenization(content.to_owned()))),
            BlockToken::Text(content) => secondary_tokens.push(SecondaryToken::Text(inline_tokenization(content.to_owned()))),
            BlockToken::Heading(heading_token) => secondary_tokens.push(SecondaryToken::Heading(heading_token.level, inline_tokenization(heading_token.value.to_owned()))),
            BlockToken::CodeBlock(cblock_token) => secondary_tokens.push(SecondaryToken::CodeBlock(cblock_token.to_owned())),
            BlockToken::List(list_token) => secondary_tokens.push(SecondaryToken::List(list_miner(list_token))),
            _ => {
                continue;
            }
        }
        token_iter.next();
    }
   
    secondary_tokens
}

pub fn parser(input: &Vec<SecondaryToken>)-> String {
    let mut iterator  = input.iter().peekable();
    let mut html_stream = String::from("<div>\n");
    let mut initi_p = false;

    while let Some(token) = iterator.peek() {
        match *token {
            SecondaryToken::Br => html_stream.push_str("<br>\n"),
            SecondaryToken::P => {
                match initi_p {
                    true => html_stream.push_str("</p>\n<p>\n"),
                    false => {
                        initi_p = true;
                        html_stream.push_str("<p>");
                    }
                    
                }
            },
            SecondaryToken::Hr(hr_token) => {
                match hr_token {
                    HrToken::Simple => html_stream.push_str("<hr class=\"hr\">\n"),
                    HrToken::Bold => {
                        html_stream.push_str("<hr style=\"height:0.2rem;\" class=\"hr\">\n")
                    },
                }
            },
            SecondaryToken::CodeBlock(cblock_token) => {
                match cblock_token {
                    CodeBlock::Open => html_stream.push_str("<div class=\"code_block\">\n"),
                    CodeBlock::Close => html_stream.push_str("</div>\n"),
                }
            },
            SecondaryToken::Text(inline_tokens) => {
                let txt_content = format!("<span>{}</span>\n", inline_tokens_parser(&mut inline_tokens.clone()));
                html_stream.push_str(&txt_content)
            },
            SecondaryToken::Blockquote(inline_tokens) => {
                let txt_content = format!("<blockquote>{}</blockquote>\n", inline_tokens_parser(&mut inline_tokens.clone()));
                html_stream.push_str(&txt_content)
            },
            SecondaryToken::Heading(level, inline_tokens) => {
                let txt_content = format!("<h{level}>{}</h{level}>\n", inline_tokens_parser(&mut inline_tokens.clone()));
                html_stream.push_str(&txt_content)
            },
            SecondaryToken::List(sec_list) => {
                let mut list_element = String::new();
                list_parser(sec_list, &mut list_element);

                list_element.push_str("</ul>\n");
                html_stream.push_str(&list_element);
            },
            _ => {
                iterator.next();
                continue;
            }
        }

        iterator.next();
    }

    html_stream.push_str("</p>\n</div>\n");
    html_stream
}

fn list_parser(sec_list: &SecList, list_element: &mut String) {
    match &sec_list.r#type {
        ListType::Todo => {
            list_element.push_str("<ul style=\"list-style-type:none;\" class=\"todo_list\">\n");
        },
        ListType::Roman(case) => {
            match case {
                block_token::RomanType::Upper => {
                    list_element.push_str("<ul style=\"list-style-type:upper_roman;\" class=\"roman_list\">\n");
                },
                block_token::RomanType::Lower => {
                    list_element.push_str("<ul style=\"list-style-type:upper_roman;\" class=\"roman_list\">\n");
                },
            }
        },
        ListType::Numbered => {
            list_element.push_str("<ul style=\"list-style-type: decimal;\" class=\"numbered_list\">\n");
        },
        ListType::Bullet(bullet_type) => {
            match bullet_type {
                block_token::BulletType::Default => {
                    list_element.push_str("<ul style=\"list-style-type: disc;\" class=\"list\">\n");
                },
                block_token::BulletType::Custom(name) => {
                    let open_tag = format!("<ul style=\"list-style-type: {0};\" class=\"{0}-list\">\n", name);
                    list_element.push_str(&open_tag);
                },
            }
        },
        ListType::Alphabetic(case) => {
            match case {
                block_token::AlphaType::Upper => {
                    list_element.push_str("<ul style=\"list-style-type: upper-alpha;\" class=\"numbered_list\">\n");
                },
                block_token::AlphaType::Lower => {
                    list_element.push_str("<ul style=\"list-style-type: lower-alpha;\" class=\"numbered_list\">\n");
                },
            }
        },
    }
    
    for (meta, tokens) in &sec_list.items {
        let mut text_content = String::new();
        match meta {
            ListMeta::None => list_element.push_str("<li>\n"),
            ListMeta::Checked => list_element.push_str("<li class=\"checked\">\n\t<input type=\"checkbox\" checked>"),
            ListMeta::Unchecked => list_element.push_str("<li class=\"unchecked\">\n\t<input type=\"checkbox\">"),
        }
        if tokens.len() > 0 {
            text_content.push_str(&inline_tokens_parser(&mut tokens.clone()));
        } else {
            list_element.push('\t');
            list_parser(&sec_list.nests[sec_list.is_nested], list_element);
        }
    }
    list_element.push_str("</li>\n");
}

pub fn position_parser(position: &InlineTokenPos, tokens: &mut Peekable<std::slice::Iter<'_, InlineToken>>, html: &mut String, fall_back_str: &str)-> Action {
    match position {
        InlineTokenPos::Open => {
            if let None = tokens.peek() {
                let txt_content = format!("<span>{}</span>", fall_back_str);
                html.push_str(&txt_content);
                return Action::Break;
            }
            else {
                return Action::Continue(true);
            }
        },
        InlineTokenPos::Close | InlineTokenPos::None => Action::Continue(false),
    }
}

pub  fn inline_tokens_parser(tokens: &mut Vec<InlineToken>)-> String {
    let mut inline_formats = String::new();
    let mut peekable = tokens.iter().peekable();

    while let Some(token) = peekable.peek() {
        match token {
            InlineToken::Text(content) => inline_formats.push_str(&content),
            InlineToken::Emoji(inline_token_pos) => {
                match position_parser(inline_token_pos, &mut peekable, &mut inline_formats, ":") {
                    Action::Break => break,
                    Action::Continue(_) => {
                        // TODO: Emoji support Implementation
                        inline_formats.push_str("#unimplimented yet#");
                        peekable.next();
                        continue;
                    }
                }
            },
            InlineToken::Code(inline_token_pos) => {
                match position_parser(inline_token_pos, &mut peekable, &mut inline_formats, "``") {
                    Action::Break => break,
                    Action::Continue(is_open) => {
                        match is_open {
                            true => inline_formats.push_str("<code class=\"code\">"),
                            false => inline_formats.push_str("</code>"),
                        }
                        peekable.next();
                        continue;
                    }
                }
            },
            InlineToken::Bold(inline_token_pos) => {
                match position_parser(inline_token_pos, &mut peekable, &mut inline_formats, "*") {
                    Action::Break => break,
                    Action::Continue(is_open) => {
                        match is_open {
                            true => inline_formats.push_str("<b class=\"bold\">"),
                            false => inline_formats.push_str("</b>"),
                        }
                        peekable.next();
                        continue;
                    }
                }
            },
            InlineToken::Strike(inline_token_pos) => {
                match position_parser(inline_token_pos, &mut peekable, &mut inline_formats, "~") {
                    Action::Break => break,
                    Action::Continue(is_open) => {
                        match is_open {
                            true => inline_formats.push_str("<strike class=\"strike\">"),
                            false => inline_formats.push_str("</strike>"),
                        }
                        peekable.next();
                        continue;
                    }
                }
            },
            InlineToken::Italic(inline_token_pos) => {
                match position_parser(inline_token_pos, &mut peekable, &mut inline_formats, "_") {
                    Action::Break => break,
                    Action::Continue(is_open) => {
                        match is_open {
                            true => inline_formats.push_str("<i class=\"italic\">"),
                            false => inline_formats.push_str("</i>"),
                        }
                        peekable.next();
                        continue;
                    }
                }
            },
            InlineToken::Caption(inline_token_pos) => {
                match position_parser(inline_token_pos, &mut peekable, &mut inline_formats, "**") {
                    Action::Break => break,
                    Action::Continue(is_open) => {
                        match is_open {
                            true => inline_formats.push_str("<caption class=\"caption\">"),
                            false => inline_formats.push_str("</caption>"),
                        }
                        peekable.next();
                        continue;
                    }
                }
            },
            InlineToken::Underline(inline_token_pos) => {
                match position_parser(inline_token_pos, &mut peekable, &mut inline_formats, "__") {
                    Action::Break => break,
                    Action::Continue(is_open) => {
                        match is_open {
                            true => inline_formats.push_str("<u class=\"underlined\">"),
                            false => inline_formats.push_str("</u>"),
                        }
                        peekable.next();
                        continue;
                    }
                }
            },
            InlineToken::Highlight(inline_token_pos) => {
                match position_parser(inline_token_pos, &mut peekable, &mut inline_formats, "==") {
                    Action::Break => break,
                    Action::Continue(is_open) => {
                        match is_open {
                            true => {
                                peekable.next();
                                if let Some(token) = peekable.peek() {
                                    match *token {
                                        InlineToken::Text(value) => {
                                            let element = format!("<span style=\"background-color: {value};\" class=\"highlight\">");
                                            inline_formats.push_str(&element);
                                            peekable.next();
                                            continue;
                                        }
                                        _ => inline_formats.push_str("==")
                                    }
                                }
                            },
                            false => inline_formats.push_str("</span>"),
                        }
                        peekable.next();
                        continue;
                    }
                }
            },
            _ => {
                peekable.next();
                continue;
            }
        }
        peekable.next();
    }

    inline_formats
}

pub fn list_miner(subject_list: &ListToken)-> SecList {
    println!("List {:?}\n", subject_list);
    let mut list = SecList {
        r#type: subject_list.r#type.clone(),
        is_nested: 0,
        items: vec![],
        nests: vec![],
    };

    for item in &subject_list.items {
        match item.value {
            block_token::ItemValue::Value(ref value) => {
                list.items.push((item.if_meta.clone(), inline_tokenization(value.clone())));
            },
            block_token::ItemValue::Nesting(index) => {
                let mut nested_list = list_miner(&subject_list.nests[index]);
                nested_list.is_nested = index;
                list.nests.push(nested_list);
                list.items.push((item.if_meta.clone(), vec![]));
            },
        }
    }

    list
}
