use std::iter::Peekable;

#[derive(Debug, Clone)]
pub struct HeadingToken{
    pub level: usize,
    pub value: String
}

#[derive(Debug, Clone)]
pub enum HrToken{
    Simple,
    Bold
}

#[derive(Debug, Clone)]
///Variants for spanning CodeBlock content
/// 
/// ```Open ``` marks the start of a code block while
/// ```Close ``` marks the CodeBlock of a code block
pub enum CodeBlock{
    Open,
    Close
}

#[derive(Debug, Clone)]
pub enum AlignmentToken {
    Left,
    Right,
    Center,
    Justify
}

#[derive(Debug, Clone)]
pub enum BlockToken {
    Br,
    P,
    Hr(HrToken),
    Text(String),
    Form(String),
    Table(String),
    List(ListToken),
    CodeBlock(CodeBlock),
    Blockquote(String),
    Heading(HeadingToken),
    Alignment(AlignmentToken),
    Image(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BulletType {
    Default,
    Custom(String)
}

#[derive(Debug, Clone)]
pub enum ListMeta {
    None,
    Checked,
    Unchecked,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ListType {
    Todo,
    Numbered,
    Roman(RomanType),
    Bullet(BulletType),
    Alphabetic(AlphaType),
}

#[derive(Debug, Clone, PartialEq)]
pub enum RomanType {
    Upper,
    Lower
}

#[derive(Debug, Clone, PartialEq)]
pub enum AlphaType {
    Upper,
    Lower
}

#[derive(Debug, Clone)]
/// Descriptive variant for whether the list value is a nesting or not
///
/// Nesting is a list that is inside another list represented by 
/// the position in the nests attribute of ```ListItem```
pub enum ItemValue {
    Value(String),
    Nesting(usize),
}

#[derive(Debug, Clone)]
pub struct ListItem {
    pub value: ItemValue,
    pub if_meta: ListMeta
}

#[derive(Debug, Clone)]
pub struct ListToken {
    pub r#type: ListType,
    pub items: Vec<ListItem>,
    pub nests: Vec<ListToken>
}

enum MultiLineFlag {
    None,
    Blockquote,
}

struct MultiLineToken {
    value: MultiLineFlag
}

impl MultiLineToken {
    fn set(&mut self, value: MultiLineFlag) {
        self.value = value;
    }
}

pub fn block_tokenization(container: &mut Vec<BlockToken>, input_lines: &Vec<&str>) {
    let mut lines_iter: Peekable<std::slice::Iter<'_, &str>> = input_lines.iter().peekable();
    let mut multi_line_id = MultiLineToken{value: MultiLineFlag::None};

    while let Some(&&line) = lines_iter.peek() {
        let mut line_chars: Peekable<std::str::Chars<'_>> = line.chars().peekable();

        // Checking for empty line
        if line.trim().is_empty() {
            container.push(BlockToken::P);
            multi_line_id.value = MultiLineFlag::None;
            lines_iter.next();
            continue;
        }

        // tokenize for Alignment
        if line.starts_with("|") {
            match line.split_at(3).0 {
                "|< " => {
                    container.push(BlockToken::Alignment(AlignmentToken::Left));
                    for _ in 0..3 {
                        line_chars.next();
                    }
                }
                "|> " => {
                    container.push(BlockToken::Alignment(AlignmentToken::Right));
                    for _ in 0..3 {
                        line_chars.next();
                    }
                }
                "|= " => {
                    container.push(BlockToken::Alignment(AlignmentToken::Center));
                    for _ in 0..3 {
                        line_chars.next();
                    }
                }
                "|- " => {
                    container.push(BlockToken::Alignment(AlignmentToken::Justify));
                    for _ in 0..3 {
                        line_chars.next();
                    }
                }
                _ => {}
            }
        }

        //Checking for multi-line flag to cater for any preveous multi-line element 
        match multi_line_id.value {
            MultiLineFlag::Blockquote => {
                let last_occur: &mut BlockToken  = container.iter_mut().rev().find(|token| {
                    match *token {
                        BlockToken::Blockquote(_) => true,
                        _ => false
                    }
                }).unwrap();

                match last_occur {
                    BlockToken::Blockquote(ref mut value) => {
                        let content = format!("<br>\n<span>{}</span>", line);
                        value.push_str(&content);
                        lines_iter.next();
                        continue;
                    },
                    _ => {}
                }
            },
            _ => {}
        }

        //Incase no preveous multiline element, continue to match any otehr block level element
        if let Some(c) = line_chars.peek() {
            match c {
                '#' => {
                    tokenize_heading(line, container);
                },
                '_' => {
                    tokenize_horizontal_line(line, container);
                },
                '>' => {
                    tokenize_blockquote(line, container, &mut multi_line_id);
                },
                '`' => {
                    tokenize_codeblock(line, container);
                }
                ' ' => {
                    let mut level = 0;
                    let mut step = line.get(level..4);
                    while let Some(part) = step {
                        if part == "    " {
                            line_chars.next();
                            for _ in 0..3 {
                                line_chars.next();
                            }
                            if let Some(nxt_char) = line_chars.peek() {
                                match nxt_char {
                                    ' ' => {
                                        level += 4;
                                        step = line.get(level..level+4);
                                        continue;
                                    }
                                    '-' => {
                                        tokenize_bulleted_list(line, container, level+4);
                                        break;
                                    }
                                    num if num.is_numeric() => {
                                        tokenize_numbered_list(line, container, level+4);
                                        break;
                                    }
                                    alpha if alpha.is_alphabetic() => {
                                        if alpha.is_uppercase() {
                                            tokenize_alpha_list(line, container, true, level+4);
                                        } else {
                                            tokenize_alpha_list(line, container, false, level+4);
                                        }
                                        break;
                                    }
                                    _ => {
                                        container.push(BlockToken::Br);
                                        container.push(BlockToken::Text(line.to_owned()));
                                        break;
                                    }
                                }
                            } else {
                                container.push(BlockToken::Br);
                                container.push(BlockToken::Text(line.to_owned()));
                                break;
                            }
                        } else {
                            container.push(BlockToken::Br);
                            container.push(BlockToken::Text(line.to_owned()));
                            break
                        }
                    }
                }
                '\t' => {
                    line_chars.next();
                    let mut level = 1;
                    while let Some(nxt_c) = line_chars.peek() {
                        match nxt_c {
                            '\t' => {
                                line_chars.next();
                                level += 1;
                            }
                            '-' => {
                                tokenize_bulleted_list(line, container, level);
                                break;
                            }
                            num if num.is_numeric() => {
                                tokenize_numbered_list(line, container, level);
                                break;
                            }
                            alpha if alpha.is_alphabetic() => {
                                if alpha.is_uppercase() {
                                    tokenize_alpha_list(line, container, true, level);
                                } else {
                                    tokenize_alpha_list(line, container, false, level);
                                }
                                continue;
                            }
                            _ => {
                                container.push(BlockToken::Br);
                                container.push(BlockToken::Text(line.to_owned()));
                                break;
                            }
                        }
                        if let None = line_chars.peek() {
                            container.push(BlockToken::Br);
                            container.push(BlockToken::Text(line.to_owned()));
                            break;
                        }
                    }
                }
                '-' => {
                    tokenize_bulleted_list(line, container, 0);
                }
                num if num.is_numeric() => {
                    tokenize_numbered_list(line, container, 0);
                }
                alpha if alpha.is_alphabetic() => {
                    if alpha.is_uppercase() {
                        tokenize_alpha_list(line, container, true, 0);
                    } else {
                        tokenize_alpha_list(line, container, false, 0);
                    }
                }
                _ => {
                    container.push(BlockToken::Br);
                    container.push(BlockToken::Text(line.to_owned()));
                }
            }
        }

        lines_iter.next();
    }
}

fn tokenize_codeblock(line: &str, container: &mut Vec<BlockToken>){
    let mut line_chars = line.chars().peekable();
    let mut level = 0;

    if line.contains(|non_target| {
        if non_target == ' ' || non_target == '`' {
            return false;
        } else {
            return true;
        }
    }) {
        container.push(BlockToken::Br);
        container.push(BlockToken::Text(line.to_owned()));
        return;
    }
    while let Some(nxt_c) = line_chars.peek() {
        match nxt_c {
            '`' => {
                if level > 3 {
                    container.push(BlockToken::Br);
                    container.push(BlockToken::Text(line.to_owned()));
                    break;
                }

                line_chars.next();
                level += 1;

                if let None = line_chars.peek() {
                    if level == 3 {

                        let last_occur = container.iter().rev().find(|token| {
                            match *token {
                                BlockToken::CodeBlock(_) => true,
                                _ => false
                            }
                        });

                        if let Some(token) = last_occur {
                            match token {
                                BlockToken::CodeBlock(inner) => {
                                    match inner {
                                        CodeBlock::Open => {
                                            container.push(BlockToken::CodeBlock(CodeBlock::Close));
                                            break;
                                        }
                                        CodeBlock::Close => {
                                            container.push(BlockToken::CodeBlock(CodeBlock::Open));
                                            break;
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        else {
                            container.push(BlockToken::CodeBlock(CodeBlock::Open));
                        }
                    } else {
                        container.push(BlockToken::Br);
                        container.push(BlockToken::Text(line.to_owned()));
                    } 
                }

            }
            ' ' => {
                if level < 3 {
                    container.push(BlockToken::Br);
                    container.push(BlockToken::Text(line.to_owned()));
                    return;
                }
                
                let last_occur = container.iter().rev().find(|token| {
                    match *token {
                        BlockToken::CodeBlock(_) => true,
                        _ => false
                    }
                });

                if let Some(token) = last_occur {
                    match token {
                        BlockToken::CodeBlock(inner) => {
                            match inner {
                                CodeBlock::Open => {
                                    container.push(BlockToken::CodeBlock(CodeBlock::Close));
                                    break;
                                },
                                CodeBlock::Close => {
                                    container.push(BlockToken::CodeBlock(CodeBlock::Open));
                                    break;
                                },
                            }
                        }
                        _ => {}
                    }
                }

                else {
                    container.push(BlockToken::CodeBlock(CodeBlock::Open));
                }
            }
            _ => {
                container.push(BlockToken::Br);
                container.push(BlockToken::Text(line.to_owned()));
                break;
            }
        }
    }
}

fn tokenize_blockquote(line: &str, container: &mut Vec<BlockToken>, multiline_flag: &mut MultiLineToken) {
    let mut line_chars = line.chars().peekable();

    line_chars.next();
    while let Some(nxt_c) = line_chars.peek() {
        match nxt_c {
            ' ' => {
                let value = line.get(2..).unwrap();
                container.push(BlockToken::Blockquote(format!("<span>{}</span>", value).to_owned()));
                multiline_flag.set(MultiLineFlag::Blockquote);
                break;
            },
            _ => {
                container.push(BlockToken::Br);
                container.push(BlockToken::Text(line.to_owned()));
                break;
            }
        }
    }

}

fn tokenize_alpha_list(line: &str, container: &mut Vec<BlockToken>, is_uppercase: bool, nesting: usize) {
    let mut line_chars = if nesting > 0 {
        line.chars().skip(nesting).peekable()
    } else {
        line.chars().skip(nesting).peekable()
    };
    let mut level = 0;
    while let Some(nxt_char) = line_chars.peek() {
        match nxt_char {
            ' '=> {
                let content = line.get(2..).unwrap();
                if let Some(token) = container.last_mut() {
                    match token {
                        BlockToken::List(inner) => {
                            if nesting > 0 {
                                let content =content.get(nesting..).unwrap();
                                
                                match inner.nests.last_mut() {
                                    Some(list) => {
                                        list.items.push(ListItem { value: ItemValue::Value(content.to_owned()), if_meta: ListMeta::None });
                                    }
                                    None => {
                                        inner.nests.push(ListToken { 
                                            r#type: ListType::Alphabetic(if is_uppercase { AlphaType::Upper } else { AlphaType::Lower }), 
                                            items: vec![ListItem { value: ItemValue::Value(content.to_owned()), if_meta: ListMeta::None }], 
                                            nests: Vec::new() 
                                        });
                                        inner.items.push(
                                            ListItem { value: ItemValue::Nesting(inner.items.len()-1), if_meta: ListMeta::None }
                                        );
                                    }
                                }
                            } else {
                                if inner.r#type != ListType::Alphabetic(if is_uppercase { AlphaType::Upper } else { AlphaType::Lower }) {
                                    let list = ListToken{
                                        r#type: ListType::Alphabetic(if is_uppercase { AlphaType::Upper } else { AlphaType::Lower }),
                                        items: vec![ListItem{value: ItemValue::Value(content.to_owned()), if_meta : ListMeta::None}],
                                        nests: vec![]
                                    };
                                    container.push(BlockToken::List(list));
                                    break;
                                }
                                inner.items.push(ListItem { value: ItemValue::Value(content.to_owned()), if_meta: ListMeta::None });
                            }
                            break;
                        }
                        _ => {
                            let list = ListToken{
                                r#type: match is_uppercase {
                                    true => ListType::Alphabetic(AlphaType::Upper),
                                    false => ListType::Alphabetic(AlphaType::Lower),
                                },
                                items: vec![ListItem{value: ItemValue::Value(content.to_owned()), if_meta: ListMeta::None}],
                                nests: vec![]
                            };
                            container.push(BlockToken::List(list));
                            break;
                        }
                    }
                } else {
                    if nesting > 0 {
                        container.push(BlockToken::Br);
                        container.push(BlockToken::Text(line.to_owned()));
                        break;
                    }
                    
                    let list = ListToken{
                        r#type: match is_uppercase {
                            true => ListType::Alphabetic(AlphaType::Upper),
                            false => ListType::Alphabetic(AlphaType::Lower),
                        },
                        items: vec![ListItem{value: ItemValue::Value(content.to_owned()), if_meta: ListMeta::None}],
                        nests: vec![]
                    };
                    container.push(BlockToken::List(list));
                    break;   
                }
            }
            '.' => {
                line_chars.next();
                level += 1;

                if let None = line_chars.peek() {
                    container.push(BlockToken::Br);
                    container.push(BlockToken::Text(line.to_owned()));
                    break;
                }
            }
            ')' => {
                line_chars.next();
                if let Some(nxt_char) = line_chars.peek(){
                    match nxt_char {
                        ' '=> {

                            continue;
                        }
                        _  => {
                            container.push(BlockToken::Br);
                            container.push(BlockToken::Text(line.to_owned()));
                            break;
                        }
                    }
                } else {
                    container.push(BlockToken::Br);
                    container.push(BlockToken::Text(line.to_owned()));
                    break;
                }
            }
            alpha if alpha.is_alphabetic() => {
                if *alpha == 'i' {
                    line_chars.next();
                    level += 1;
                    continue;
                }
                line_chars.next();
                level += 1;
                if level > 1 {
                    container.push(BlockToken::Br);
                    container.push(BlockToken::Text(line.to_owned()));
                    break;
                }
            }
            _ => {
                container.push(BlockToken::Br);
                container.push(BlockToken::Text(line.to_owned()));
                break;
            }
        }
    }
}

fn tokenize_numbered_list(line: &str, container: &mut Vec<BlockToken>, nesting: usize) {
    let mut line_chars = if nesting > 0 {
        line.chars().skip(nesting).peekable()
    } else {
        line.chars().skip(0).peekable()
    };
    let mut level = 0;
    while let Some(nxt_char) = line_chars.peek() {
        match nxt_char {
            '.' => {
                line_chars.next();
                if let Some(nxt_char) = line_chars.peek(){
                    match nxt_char {
                        ' '=> {
                            let content = line.get(level+2..).unwrap();
                            if let Some(token) = container.last_mut() {
                                match token {
                                    BlockToken::List(list_token) => {
                                        if nesting > 0 {
                                            let content = content.get(nesting..).unwrap();
                                            match list_token.nests.last_mut() {
                                                Some(list) => {
                                                    list.items.push(ListItem { value: ItemValue::Value(content.to_owned()), if_meta: ListMeta::None });
                                                }
                                                None => {
                                                    list_token.nests.push(ListToken { 
                                                        r#type: ListType::Numbered, 
                                                        items: vec![ListItem { value: ItemValue::Value(content.to_owned()), if_meta: ListMeta::None }], 
                                                        nests: Vec::new() 
                                                    });
                                                    list_token.items.push(
                                                        ListItem { value: ItemValue::Nesting(list_token.items.len()-1), if_meta: ListMeta::None }
                                                    );
                                                }
                                            }
                                        } else {
                                            if list_token.r#type != ListType::Numbered {
                                                let list = ListToken{
                                                    r#type: ListType::Numbered,
                                                    items: vec![ListItem{value: ItemValue::Value(content.to_owned()), if_meta: ListMeta::None}],
                                                    nests: vec![]
                                                };
                                                container.push(BlockToken::List(list));
                                                break;
                                            } else {
                                                list_token.items.push(ListItem { value: ItemValue::Value(content.to_owned()), if_meta: ListMeta::None });
                                            }
                                        }
                                        break;
                                    },
                                    _ => {
                                        let list = ListToken{
                                            r#type: ListType::Numbered,
                                            items: vec![ListItem{value: ItemValue::Value(content.to_owned()), if_meta : ListMeta::None}],
                                            nests: vec![]
                                        };
                                        container.push(BlockToken::List(list));
                                        break;
                                    }
                                } 
                            } else {
                                if nesting > 0 {
                                    container.push(BlockToken::Br);
                                    container.push(BlockToken::Text(line.to_owned()));
                                    break;
                                }

                                let list = ListToken{
                                    r#type: ListType::Numbered,
                                    items: vec![ListItem{value: ItemValue::Value(content.to_owned()), if_meta : ListMeta::None}],
                                    nests: vec![]
                                };
                                container.push(BlockToken::List(list));
                                break;
                            }
                        }
                        _  => {
                            container.push(BlockToken::Br);
                            container.push(BlockToken::Text(line.to_owned()));
                            break;
                        }
                    }
                }
            }
            num if num.is_numeric() => {
                line_chars.next();
                level += 1;
            }
            _ => {
                container.push(BlockToken::Br);
                container.push(BlockToken::Text(line.to_owned()));
                break;
            }
        }
    }
}

fn tokenize_bulleted_list(line: &str, container: &mut Vec<BlockToken>, nesting: usize) {
    let mut line_chars = if nesting > 0 {
        line.chars().skip(nesting).peekable()
    } else {
        line.chars().skip(0).peekable()
    };

    let mut level = 0;
    let mut list_type = ListType::Bullet(BulletType::Default);
    let mut list_meta = ListMeta::None;
    
    while let Some(nxt_char) = line_chars.peek() {
        match nxt_char {
            ' ' => {
                let content = line.get(level+2..).unwrap();
                if let Some(token) = container.last_mut() {
                    match token {
                        BlockToken::List(list_token) => {
                            if nesting > 0 {
                                let content = content.get(nesting..).unwrap();
                                match list_token.nests.last_mut() {
                                    Some(list) => {
                                        list.items.push(ListItem { value: ItemValue::Value(content.to_owned()), if_meta: list_meta });
                                    }
                                    None => {
                                        list_token.nests.push(ListToken { 
                                            r#type: ListType::Bullet(BulletType::Default), 
                                            items: vec![ListItem { value: ItemValue::Value(content.to_owned()), if_meta: list_meta }], 
                                            nests: Vec::new() 
                                        });
                                        list_token.items.push(
                                            ListItem { value: ItemValue::Nesting(list_token.items.len()-1), if_meta: ListMeta::None }
                                        );
                                    }
                                }
                            } else {
                                if list_token.r#type != list_type {
                                    let list = ListToken{
                                        r#type: list_type,
                                        items: vec![ListItem{value: ItemValue::Value(content.to_owned()), if_meta: list_meta}],
                                        nests: vec![]
                                    };
                                    container.push(BlockToken::List(list));
                                    break;
                                }
                                list_token.items.push(ListItem { value: ItemValue::Value(content.to_owned()), if_meta: list_meta });
                            }
                            break;
                        },
                        _ => {
                            let list = ListToken{
                                r#type: list_type,
                                items: vec![ListItem{value: ItemValue::Value(content.to_owned()), if_meta : list_meta}],
                                nests: vec![]
                            };
                            container.push(BlockToken::List(list));
                            break;
                        }
                    }
                } else {
                    if nesting > 0 {
                        container.push(BlockToken::Br);
                        container.push(BlockToken::Text(line.to_owned()));
                        break;
                    }

                    let list = ListToken{
                        r#type: ListType::Bullet(BulletType::Default),
                        items: vec![ListItem{value: ItemValue::Value(content.to_owned()), if_meta : ListMeta::None}],
                        nests: vec![]
                    };
                    container.push(BlockToken::List(list));
                    break;
                }
            }
            '[' => {
                line_chars.next();
                level += 1;
                if let Some(nxt_char) = line_chars.peek() {
                    match nxt_char {
                        sign if *sign == 'x' || *sign == 'X' || *sign == ' ' => {
                            let mut kind = ListMeta::Checked;
                            if *sign == ' ' {
                                kind = ListMeta::Unchecked
                            }
                            line_chars.next();
                            level += 1;
                            if let Some(nxt_char) = line_chars.peek() {
                                match nxt_char {
                                    ']' => {
                                        line_chars.next();
                                        if let Some(nxt_char) = line_chars.peek() {
                                            match nxt_char {
                                                ' ' => {
                                                    list_type = ListType::Todo;
                                                    list_meta = kind;
                                                    continue;
                                                }
                                                _ => {
                                                    container.push(BlockToken::Br);
                                                    container.push(BlockToken::Text(line.to_owned()));
                                                    break; 
                                                }
                                            }
                                        } else {
                                            container.push(BlockToken::Br);
                                            container.push(BlockToken::Text(line.to_owned()));
                                            break; 
                                        }
                                    }
                                    _ => {
                                        container.push(BlockToken::Br);
                                        container.push(BlockToken::Text(line.to_owned()));
                                        break; 
                                    }
                                }  
                            } else {
                                container.push(BlockToken::Br);
                                container.push(BlockToken::Text(line.to_owned()));
                                break; 
                            }
                        }
                        _ => {
                            container.push(BlockToken::Br);
                            container.push(BlockToken::Text(line.to_owned()));
                            break; 
                        }
                    }
                } else {
                    container.push(BlockToken::Br);
                    container.push(BlockToken::Text(line.to_owned()));
                    break;   
                }
            }
            '-' => {
                if level > 1 {
                    container.push(BlockToken::Br);
                    container.push(BlockToken::Text(line.to_owned()));
                    break;
                }
                line_chars.next();
                level += 1;
                if let None = line_chars.peek() {
                    container.push(BlockToken::Br);
                    container.push(BlockToken::Text(line.to_owned()));
                    break;
                }
                continue
            }
            '(' => {
                line_chars.next();
                level += 1;
                let mut bullet_buffer = String::new();
                while let Some(nxt_char) = line_chars.peek() {
                    match nxt_char {
                        ')' => {
                            line_chars.next();
                            if let Some(nxt_char) = line_chars.peek() {
                                match nxt_char {
                                    ' ' => {
                                        list_type = ListType::Bullet(BulletType::Custom(bullet_buffer.clone()));
                                        list_meta = ListMeta::None;
                                        continue;
                                    }
                                    _ => {
                                        container.push(BlockToken::Br);
                                        container.push(BlockToken::Text(line.to_owned()));
                                        break; 
                                    }
                                }
                            } else {
                                container.push(BlockToken::Br);
                                container.push(BlockToken::Text(line.to_owned()));
                                break; 
                            }
                        }
                        _ => {
                            if bullet_buffer.len() > 10 {
                                container.push(BlockToken::Br);
                                container.push(BlockToken::Text(line.to_owned()));
                                break; 
                            }
                            bullet_buffer.push(*nxt_char);
                            line_chars.next();
                            if let None = line_chars.peek() {
                                container.push(BlockToken::Br);
                                container.push(BlockToken::Text(line.to_owned()));
                                break; 
                            }
                        }
                    }
                }
            }
            _ => {
                container.push(BlockToken::Br);
                container.push(BlockToken::Text(line.to_owned()));
                break;
            }
        }
    }
}

fn tokenize_horizontal_line(line: &str, container: &mut Vec<BlockToken>) {
    let mut line_chars = line.chars().peekable();
    if line.contains(|non_hr: char|{
        match non_hr {
            target if target == '_' || target.is_ascii_whitespace() => false,
            _ => true
        }
    }) {
        container.push(BlockToken::Br);
        container.push(BlockToken::Text(line.to_owned()));
        return;
    }
    let mut level = 0;
    while let Some(nxt_c) = line_chars.peek() {
        match nxt_c {
            '_' => {
                if level > 4 {
                    container.push(BlockToken::Br);
                    container.push(BlockToken::Text(line.to_owned()));
                    break;
                }
                line_chars.next();
                level += 1;
                if let None = line_chars.peek() {
                    if level >= 3{ 
                        if level == 3 {
                            container.push(BlockToken::Hr(HrToken::Simple));
                        } else if level == 4 {
                            container.push(BlockToken::Hr(HrToken::Bold));
                        }
                    } else {
                        container.push(BlockToken::Br);
                        container.push(BlockToken::Text(line.to_owned()));
                    }
                }
            },
            ' ' => {
                if level > 2 {
                    if level == 3 {
                        container.push(BlockToken::Hr(HrToken::Simple));
                    } else if level == 4 {
                        container.push(BlockToken::Hr(HrToken::Bold));
                    }
                } else {
                    container.push(BlockToken::Br);
                    container.push(BlockToken::Text(line.to_owned()));
                }
                break;
            }
            _ => {
                container.push(BlockToken::Br);
                container.push(BlockToken::Text(line.to_owned()));
                break;
            }
        }
    }

}

fn tokenize_heading(line: &str, container: &mut Vec<BlockToken>) {
    let mut line_chars = line.chars().peekable();
    let mut level = 0;
    while let Some(&nxt_c) = line_chars.peek() {
        match nxt_c {
            ' ' => {
                let value = line.get(level+1..).unwrap().to_owned();
                container.push(BlockToken::Heading(
                    HeadingToken { level, value }
                ));
                break;
            },
            '#' => {
                if level > 6 {
                    container.push(BlockToken::Br);
                    container.push(BlockToken::Text(line.to_owned()));
                    break;
                }
                level += 1;
                line_chars.next();
            },
            _ => {
                container.push(BlockToken::Br);
                container.push(BlockToken::Text(line.to_owned()));
                break;
            }
        }
    }
}
