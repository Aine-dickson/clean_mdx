use std::iter::Peekable;

#[derive(Debug, Clone)]
/// Specifies whether the inline token is a close or open token on the given formating
pub enum InlineTokenPos {
    Open,
    Close,
    None
}

#[derive(Debug, Clone)]
pub enum InlineToken {
    Text(String), Link(String), Emoji(InlineTokenPos),
    Code(InlineTokenPos), Bold(InlineTokenPos),
    Strike(InlineTokenPos), Italic(InlineTokenPos),
    Caption(InlineTokenPos), Underline(InlineTokenPos),
    Highlight(InlineTokenPos),
    Size(), Color(), Family(),
}

impl InlineToken {
    ///Returns contained variant(```InlineTokenPos::Open``` or ```InlineTokenPos::Close```) of subjected token
    pub fn get_pos(&self)-> &InlineTokenPos {
        match self {
            InlineToken::Code(pos) | InlineToken::Bold(pos) |
            InlineToken::Strike(pos) | InlineToken::Italic(pos) |
            InlineToken::Caption(pos) | InlineToken::Underline(pos) |
            InlineToken::Highlight(pos) => pos,
            _ => &InlineTokenPos::None
        }
    }

    ///Returns it's corresponding ```InlineId```
    pub fn get_id(&self)-> InlineId {
        match self {
            InlineToken::Text(_) => InlineId::Text, InlineToken::Code(_) => InlineId::Code,
            InlineToken::Bold(_) => InlineId::Bold, InlineToken::Link(_) => InlineId::Link,
            InlineToken::Size() => InlineId::Size, InlineToken::Color() => InlineId::Color,
            InlineToken::Family() => InlineId::Family, InlineToken::Strike(_) => InlineId::Strike,
            InlineToken::Italic(_) => InlineId::Italic, InlineToken::Caption(_) => InlineId::Caption,
            InlineToken::Underline(_) => InlineId::Underline, InlineToken::Highlight(_) => InlineId::Highlight,
            InlineToken::Emoji(_) => InlineId::Emoji
        }
    }
}

#[derive(PartialEq, Clone)]
/// Atomic identifiers for the supported Inline tokens
pub enum InlineId {
    Text, Code, Bold,
    Link, Size, Color,
    Family, Strike, Italic,
    Caption, Underline, Emoji,
    Highlight
}

/// A specifier type for distinguishing between ```Text``` token and other **Inline** tokens
/// when pushing to the stack with aid of any general purpose pusher
pub enum Push {
    Text(String),
    Other(InlineTokenPos)
}

/// The result from rule exection used to raise concerns for possible
/// defected token pushes
pub enum RuleResult{
    Success,
    Failure
}

///Transforms the given ```id``` type to corresponding string repeated ```count``` times with no space
/// 
/// Forexample;
/// ```
/// let text = id2text_format(InlineId::Italic, 2);
/// ```
/// text would be **"__"**
pub fn id2text_format(id: &InlineId, count: usize)-> String {
    match id {
        InlineId::Text => String::from("".repeat(count)),
        InlineId::Code => String::from("`".repeat(count)),
        InlineId::Bold | InlineId::Caption => String::from("*".repeat(count)),
        InlineId::Italic | InlineId::Underline => String::from("_".repeat(count)),
        InlineId::Highlight => String::from("=".repeat(count)),
        InlineId::Strike => String::from("~".repeat(count)),
        InlineId::Emoji => String::from(":".repeat(count)),

        // TODO: Implement with corresponding identifier text for font style and linkd
        InlineId::Link => String::from("".repeat(count)),
        InlineId::Size | InlineId::Color | InlineId::Family => String::from("".repeat(count)),
    }
}

///Searches the stack for the most recent given contexxt token and checks if it's a close variant
/// 
/// If found, it returns ```true``` else returns ```false```
pub fn open_cxt_checkup<'a>(rev_stack: &'a mut Vec<InlineToken>, cxt: &'a InlineId)->bool {
    match cxt {
        InlineId::Code => match rev_stack.iter().find(|token| { match *token { InlineToken::Code(_) => true,_ => false }}){
            Some(token) => match token.get_pos() { InlineTokenPos::Open => true, InlineTokenPos::Close | InlineTokenPos::None => false },
            None => false,
        },
        InlineId::Bold => match rev_stack.iter().find(|token| { match *token { InlineToken::Bold(_) => true,_ => false }}){
            Some(token) => match token.get_pos() { InlineTokenPos::Open => true, InlineTokenPos::Close | InlineTokenPos::None => false },
            None => false,
        },
        InlineId::Caption => match rev_stack.iter().find(|token| { match *token { InlineToken::Caption(_) => true,_ => false }}){
            Some(token) => match token.get_pos() { InlineTokenPos::Open => true, InlineTokenPos::Close | InlineTokenPos::None => false },
            None => false,
        },
        InlineId::Strike => match rev_stack.iter().find(|token| { match *token { InlineToken::Strike(_) => true,_ => false }}){
            Some(token) => match token.get_pos() { InlineTokenPos::Open => true, InlineTokenPos::Close | InlineTokenPos::None => false },
            None => false,
        },
        InlineId::Italic => match rev_stack.iter().find(|token| { match *token { InlineToken::Italic(_) => true,_ => false }}){
            Some(token) => match token.get_pos() { InlineTokenPos::Open => true, InlineTokenPos::Close | InlineTokenPos::None => false },
            None => false,
        },
        InlineId::Underline => match rev_stack.iter().find(|token| { match *token { InlineToken::Underline(_) => true,_ => false }}){
            Some(token) => match token.get_pos() { InlineTokenPos::Open => true, InlineTokenPos::Close | InlineTokenPos::None => false },
            None => false,
        },
        InlineId::Emoji => match rev_stack.iter().find(|token| { match *token { InlineToken::Emoji(_) => true,_ => false }}){
            Some(token) => match token.get_pos() { InlineTokenPos::Open => true, InlineTokenPos::Close | InlineTokenPos::None => false },
            None => false,
        },
        // TODO: Implement other variants
        _ => match rev_stack.iter().find(|token| { match *token { InlineToken::Text(_) => true,_ => false }}){
            Some(token) => match token.get_pos() { InlineTokenPos::Open => true, InlineTokenPos::Close | InlineTokenPos::None => false },
            None => false,
        },
    }
}

/// General pusporse stack pusher for **Inline** tokens given the tokenization context
/// Takes arguments;
/// - ```cxt``` which is the context of type ```InlineId``` with a lifetime ```'a``` specifying the tokenization context
/// - ```stack``` which must be a mutable reference to a vector collection of **Inline** tokens
/// - ```to_push``` a type specifier for whether to push a **Text** token or any another **Inline** token type

pub fn cxt_stack_push<'a>(cxt: &'a InlineId, stack: &mut Vec<InlineToken>, to_push: Push) {
    match cxt {
        InlineId::Code => match to_push {
            Push::Other(pos) => stack.push(InlineToken::Code(pos)),
            Push::Text(value) => stack.push(InlineToken::Text(value))
        },
        InlineId::Bold => match to_push {
            Push::Other(pos) => stack.push(InlineToken::Bold(pos)),
            Push::Text(value) => stack.push(InlineToken::Text(value))
        },
        InlineId::Strike => match to_push {
            Push::Other(pos) => stack.push(InlineToken::Strike(pos)),
            Push::Text(value) => stack.push(InlineToken::Text(value))
        },
        InlineId::Italic => match to_push {
            Push::Other(pos) => stack.push(InlineToken::Italic(pos)),
            Push::Text(value) => stack.push(InlineToken::Text(value))
        },
        InlineId::Caption => match to_push {
            Push::Other(pos) => stack.push(InlineToken::Caption(pos)),
            Push::Text(value) => stack.push(InlineToken::Text(value))
        },
        InlineId::Underline => match to_push {
            Push::Other(pos) => stack.push(InlineToken::Underline(pos)),
            Push::Text(value) => stack.push(InlineToken::Text(value))
        },
        InlineId::Emoji => match to_push {
            Push::Other(pos) => stack.push(InlineToken::Emoji(pos)),
            Push::Text(value) => stack.push(InlineToken::Text(value))
        },
        _ => match to_push {
            Push::Other(_) => stack.push(InlineToken::Text("Unimplemented".to_owned())),
            Push::Text(value) => stack.push(InlineToken::Text(value))
        }
    }
}

/// The rule enforced when **whitespace** or **punctuation** that is comma, fullstop, hyphene, quote, ... character is encountered next to a character reserved for formatting
/// 
/// Takes arguments;
/// - ```stack``` which must be a mutable reference to a vector collection of **Inline** tokens
/// - ```content_chars``` which must be a mutable reference to a type ```Peekable<std::str::Chars<'_>>```
/// - ```buffer``` which must be a mutable reference to a string
/// - ```cxt``` which is the context of type ```InlineId``` with a lifetime ```'a``` specifying the tokenization context
/// - ```movements``` this specifies the steps from top tokenization the anlyser has moved (consumed characters)
pub fn nxt_is_whitespace_rule(stack: &mut Vec<InlineToken>, content_chars: &mut Peekable<std::str::Chars<'_>>, buffer: &mut String, cxt: InlineId, movements: usize){
    let nxt_char = content_chars.peek().unwrap();

    //Where stack is empty and nothing in the buffer, the token is text
    if stack.len() == 0 && buffer.clone().chars().count() == 0 {
        let mut text = id2text_format(&cxt, movements);
        text.push(*nxt_char);
        cxt_stack_push(&cxt, stack, Push::Text(text));
        content_chars.next();
        return;
    }
    if buffer.clone().chars().count() > 0 {
        cxt_stack_push(&cxt, stack, Push::Text(buffer.clone()));
        buffer.clear();
    }

    // Gettting the stack top value for analysis
    if let Some(token) = stack.get(stack.len()-1) {
        match token {
            InlineToken::Text(content) => {
                if content.clone().chars().count() == 0 {
                    cxt_stack_push(&cxt, stack, Push::Text(id2text_format(&cxt, movements)));
                    return;
                }
                match content.clone().chars().nth(content.clone().chars().count()-1).unwrap() {
                    ' ' => {
                        cxt_stack_push(&cxt, stack, Push::Text(id2text_format(&cxt, movements)));
                    },
                    // If anything else except whitespace, check for open bold
                    _ => {
                        let mut rev_stack = stack.clone();
                        rev_stack.reverse();
                        // Findout whether there's an open cxt token
                        let open_checkup_result = open_cxt_checkup(&mut rev_stack, &cxt);

                        // If no cxt token in stack yet, then it's a text token
                        match open_checkup_result {
                            true => cxt_stack_push(&cxt, stack, Push::Other(InlineTokenPos::Close)),
                            false => cxt_stack_push(&cxt, stack, Push::Text(id2text_format(&cxt, movements))),
                        }
                    }
                }
            }
            _ => {
                let mut rev_stack = stack.clone();
                rev_stack.reverse();

                // Findout whether there's an open Bold token
                // If no bold token in stack yet, then it's a text token
                match open_cxt_checkup(&mut rev_stack, &cxt) {
                    true => cxt_stack_push(&cxt, stack, Push::Other(InlineTokenPos::Close)),
                    false => cxt_stack_push(&cxt, stack, Push::Text(id2text_format(&cxt, movements))),
                }
                content_chars.next();
            }
        }
    }
}

/// The rule enforced when either a **non whitespace** character or **another reserved formatting
/// character** is encountered next to the current reserved formatting character. 
/// 
/// Returns ```RuleResult``` used for reporting concerns for unrequired close tokens
/// 
/// Takes arguments;
/// - ```stack``` which must be a mutable reference to a vector collection of **Inline** tokens
/// - ```buffer``` which must be a mutable reference to a string
/// - ```cxt``` which is the context of type ```InlineId``` with a lifetime ```'a``` specifying the tokenization context
/// - ```movements``` this specifies the steps from top tokenization the anlyser has moved (consumed characters)
/// - ```close_possibility``` specifies whether this is possibly a close token if the stack top value is text with
/// the last character not whitespace or is a close token too
pub fn nxt_non_whitespace_rule<'a>(stack: &mut Vec<InlineToken>, buffer: &mut String, cxt: &'a InlineId, movements: usize, close_possibility: bool)->RuleResult {
    let mut can_nest = [InlineId::Bold, InlineId::Caption, InlineId::Italic, InlineId::Underline, InlineId::Strike];
    for (i, id) in can_nest.iter().enumerate() {
        if id == cxt {
            can_nest[i] = InlineId::Text;
            break;
        }
    }

    //Where stack is empty and nothing in the buffer, the token is text
    if stack.len() == 0 && buffer.clone().chars().count() == 0    {
        cxt_stack_push(cxt, stack, Push::Other(InlineTokenPos::Open));
        return RuleResult::Failure;
    }
    if buffer.clone().chars().count() > 0 {
        cxt_stack_push(cxt, stack, Push::Text(buffer.clone()));
        buffer.clear();
    }

    // This is only intrested in text token to check fot last character in it
    // Or open formats for either Italic, Underline, Caption, Alignment or Highlight
    if let Some(token) = stack.get(stack.len()-1) {
        match token {
            InlineToken::Text(content) => {
                if content.clone().chars().count() == 0 {
                    cxt_stack_push(cxt, stack, Push::Text(id2text_format(&cxt, movements)));
                    return RuleResult::Failure;
                }
                match content.clone().chars().nth(content.clone().chars().count()-1).unwrap() {
                    ' '=> {
                        cxt_stack_push(cxt, stack, Push::Other(InlineTokenPos::Open));
                        return RuleResult::Failure;
                    },
                    _ => {
                        if close_possibility {
                            cxt_stack_push(cxt, stack, Push::Other(InlineTokenPos::Close));
                            return RuleResult::Success;
                        } 
                        cxt_stack_push(cxt, stack, Push::Text(id2text_format(&cxt, movements)));
                        return RuleResult::Failure;
                    },
                }
            },

            nest if can_nest.contains(&nest.get_id()) => {
                match nest.get_pos() {
                    InlineTokenPos::Open => {
                        cxt_stack_push(cxt, stack, Push::Other(InlineTokenPos::Open));
                        return RuleResult::Failure;
                    },
                    InlineTokenPos::Close => {
                        if close_possibility {
                            cxt_stack_push(cxt, stack, Push::Other(InlineTokenPos::Close));
                            return RuleResult::Success;
                        }
                        cxt_stack_push(cxt, stack, Push::Text(id2text_format(&cxt, movements)));
                        return RuleResult::Failure;
                    },
                    InlineTokenPos::None => return RuleResult::Failure
                }
            }

            _ => {
                cxt_stack_push(cxt, stack, Push::Text(id2text_format(&cxt, movements)));
                return RuleResult::Success;
            }
        }
    } else {
        return RuleResult::Failure;
    }
}

///The rule enforced when the **reserved formatting character** is encountered at the end of a current line
/// being tokenized
/// 
/// Takes arguments;
/// - ```stack``` which must be a mutable reference to a vector collection of **Inline** tokens
/// - ```buffer``` which must be a mutable reference to a string
/// - ```cxt``` which is the context of type ```InlineId``` with a lifetime ```'a``` specifying the tokenization context
/// - ```movements``` this specifies the steps from top tokenization the anlyser has moved (consumed characters)
pub fn is_last_char_rule<'a>(stack: &mut Vec<InlineToken>, buffer: &mut String, cxt: &'a InlineId, movements: usize) {
    let mut can_nest = [InlineId::Bold, InlineId::Caption, InlineId::Italic, InlineId::Underline, InlineId::Strike];
    for (i, id) in can_nest.iter().enumerate() {
        if id == cxt {
            can_nest[i] = InlineId::Text;
            break;
        }
    }

    //Where stack is empty, the token is text
    if stack.len() == 0 && buffer.clone().chars().count() == 0 {
        cxt_stack_push(cxt, stack, Push::Text(id2text_format(&cxt, movements)));
        return;
    }
    if buffer.clone().chars().count() > 0 {
        cxt_stack_push(cxt, stack, Push::Text(buffer.clone()));
        buffer.clear();
    }
    
    // For a non empty stack, enforce that if the top stack value is text, 
    // then, it's last character is not whitespace or else push a Text to stack top
    if let Some(token) = stack.get(stack.len()-1) {
        match token {
            InlineToken::Text(content) => {
                match content.clone().chars().nth(content.chars().count()-1).unwrap() {
                    ' ' => {
                        cxt_stack_push(cxt, stack, Push::Text(id2text_format(&cxt, movements)));
                        return;
                    }
                    _ => {
                        let mut reverse_stack = stack.clone();
                        reverse_stack.reverse();

                        // If no cxt token in stack yet, then it's a text token
                        match open_cxt_checkup(&mut reverse_stack, cxt) {
                            true => cxt_stack_push(cxt, stack, Push::Other(InlineTokenPos::Close)),
                            false => cxt_stack_push(cxt, stack, Push::Text(id2text_format(&cxt, movements))),
                        }
                        return;
                    }
                }
            },

            nest if can_nest.contains(&nest.get_id()) => {
                match nest.get_pos() {
                    InlineTokenPos::Open => cxt_stack_push(cxt, stack, Push::Text(id2text_format(&cxt, movements))),
                    InlineTokenPos::Close => cxt_stack_push(cxt, stack, Push::Other(InlineTokenPos::Close)),
                    InlineTokenPos::None => {}
                }
                return;
            },

            _ => {
                cxt_stack_push(cxt, stack, Push::Text(id2text_format(&cxt, movements)));
                return;
            }
        }
    }
}

/// The global formater for inline tokens. It takes the line/string to be formatted as ```block_content``` argument
/// 
/// Returns a vector collection of contained **Inline** tokens
pub fn inline_tokenization(block_content: String)-> Vec<InlineToken> {
    let mut content_chars: Peekable<std::str::Chars<'_>> = block_content.chars().peekable();

    let mut stack: Vec<InlineToken> = Vec::new();
    let mut buffer = String::new();
    let mut concerns = vec![];
    let reserved_tokens = ['~', '*', '_', ':'];
    let punctuations = [',', '.', '?', '!', '\'', '\"', ';', '-'];

    while let Some(nxt_char) = content_chars.peek() {
        match nxt_char {
            '*' => {
                // Step to next character to verify for either bold or caption
                content_chars.next();
                if let Some(post_char) = content_chars.peek() {
                    match post_char {
                        // Check for caption
                        '*' => {
                            // Step to next character to enforce no whitespace rule for caption
                            content_chars.next();
                            if let Some(decisive) = content_chars.peek() {
                                match decisive {
                                    punctuation if punctuations.contains(punctuation) | punctuation.is_whitespace() => {
                                        nxt_is_whitespace_rule(&mut stack, &mut content_chars, &mut buffer, InlineId::Caption, 2);
                                    }
                                    reserved if reserved_tokens.contains(reserved) => {
                                        match nxt_non_whitespace_rule(&mut stack, &mut buffer, &InlineId::Caption, 2, true) {
                                            RuleResult::Success => concerns.push(stack.len()),
                                            RuleResult::Failure => {},
                                        }
                                    }
                                    _ => {
                                        nxt_non_whitespace_rule(&mut stack, &mut buffer, &InlineId::Caption, 2, false);
                                    }
                                }
                            }
                            // If no next character verify if this is a close caption or text token
                            else {
                                is_last_char_rule(&mut stack, &mut buffer, &InlineId::Caption, 2);
                            }
                        }

                        // Ensures this is not an open bold and checks if it's close
                        // That's if the last character of the stack top value is not a 
                        // whitespace
                        punctuation if punctuations.contains(punctuation) | punctuation.is_whitespace() => {
                            nxt_is_whitespace_rule(&mut stack, &mut content_chars, &mut buffer, InlineId::Bold, 1);
                        }
                        
                        // Checks if top stack value whitespace or it's an open of either
                        // Italic, Underline, Caption, Alignment or Highlight
                        reserved if reserved_tokens.contains(reserved) => {
                            match nxt_non_whitespace_rule(&mut stack, &mut buffer, &InlineId::Bold, 1, true) {
                                RuleResult::Success => concerns.push(stack.len()),
                                RuleResult::Failure => {},
                            }
                        }
                        _ => {
                            nxt_non_whitespace_rule(&mut stack, &mut buffer, &InlineId::Bold, 1, false);
                        }
                    }
                }
                // If no next character, it's either a close bold or just text
                else {
                    is_last_char_rule(&mut stack, &mut buffer, &InlineId::Bold, 1);
                }
            }
            '_' => {
                // Step to next character to verify for either italic or Underline
                content_chars.next();
                if let Some(post_char) = content_chars.peek() {
                    match post_char {
                        // Check for Underline
                        '_' => {
                            // Step to next character to enforce no whitespace rule for Underline
                            content_chars.next();
                            if let Some(decisive) = content_chars.peek() {
                                match decisive {
                                    punctuation if punctuations.contains(punctuation) | punctuation.is_whitespace() => {
                                        nxt_is_whitespace_rule(&mut stack, &mut content_chars, &mut buffer, InlineId::Underline, 2);
                                    }
                                    reserved if reserved_tokens.contains(reserved) => {
                                        match nxt_non_whitespace_rule(&mut stack, &mut buffer, &InlineId::Underline, 2, true) {
                                            RuleResult::Success => concerns.push(stack.len()),
                                            RuleResult::Failure => {},
                                        }
                                    }
                                    _ => {
                                        nxt_non_whitespace_rule(&mut stack, &mut buffer, &InlineId::Underline, 2, false);
                                    }
                                }
                            }
                            // If no next character verify if this is a close caption or text token
                            else {
                                is_last_char_rule(&mut stack, &mut buffer, &InlineId::Underline, 2);
                            }
                        }

                        // Ensures this is not an open italic and checks if it's close
                        // That's if the last character of the stack top value is not a 
                        // whitespace
                        punctuation if punctuations.contains(punctuation) | punctuation.is_whitespace() => {
                            nxt_is_whitespace_rule(&mut stack, &mut content_chars, &mut buffer, InlineId::Italic, 1);
                        }
                        reserved if reserved_tokens.contains(reserved) => {
                            match nxt_non_whitespace_rule(&mut stack, &mut buffer, &InlineId::Italic, 1, true) {
                                RuleResult::Success => concerns.push(stack.len()),
                                RuleResult::Failure => {},
                            }
                        }
                        // Checks if top stack value whitespace or it's an open of either
                        // Italic, Underline, Caption, Alignment or Highlight
                        _ => {
                            nxt_non_whitespace_rule(&mut stack, &mut buffer, &InlineId::Italic, 1, false);
                        }
                    }
                }
                // If no next character, it's either a close italic or just text
                else {
                    is_last_char_rule(&mut stack, &mut buffer, &InlineId::Italic, 1);
                }
            }
            '~' => {
                // Step to next character to verify for strike through
                content_chars.next();
                if let Some(post_char) = content_chars.peek() {
                    match post_char {
                        // Ensures this is not an open Strike and checks if it's close
                        // That's if the last character of the stack top value is not a 
                        // whitespace
                        punctuation if punctuations.contains(punctuation) | punctuation.is_whitespace() => {
                            nxt_is_whitespace_rule(&mut stack, &mut content_chars, &mut buffer, InlineId::Strike, 1);
                        }
                        reserved if reserved_tokens.contains(reserved) => {
                            match nxt_non_whitespace_rule(&mut stack, &mut buffer, &InlineId::Strike, 1, true) {
                                RuleResult::Success => concerns.push(stack.len()),
                                RuleResult::Failure => {},
                            }
                        }
                        // Checks if top stack value whitespace or it's an open of either
                        // Italic, Underline, Caption, Alignment or Highlight
                        _ => {
                            nxt_non_whitespace_rule(&mut stack, &mut buffer, &InlineId::Strike, 1, false);
                        }
                    }
                }
                // If no next character, it's either a close Strike or just text
                else {
                    is_last_char_rule(&mut stack, &mut buffer, &InlineId::Strike, 1);
                }
            }
            ':' => {
                // Step to next character to verify for Emoji
                content_chars.next();
                if let Some(post_char) = content_chars.peek() {
                    match post_char {
                        // Ensures this is not an open Emoji and checks if it's close
                        // That's if the last character of the stack top value is not a 
                        // whitespace
                        punctuation if punctuations.contains(punctuation) | punctuation.is_whitespace() => {
                            nxt_is_whitespace_rule(&mut stack, &mut content_chars, &mut buffer, InlineId::Emoji, 1);
                        }
                        reserved if reserved_tokens.contains(reserved) => {
                            match nxt_non_whitespace_rule(&mut stack, &mut buffer, &InlineId::Emoji, 1, true) {
                                RuleResult::Success => concerns.push(stack.len()),
                                RuleResult::Failure => {},
                            }
                        }
                        // Checks if top stack value whitespace or it's an open of either
                        // Italic, Underline, Caption, Alignment or Highlight
                        _ => {
                            nxt_non_whitespace_rule(&mut stack, &mut buffer, &InlineId::Emoji, 1, false);
                        }
                    }
                }
                // If no next character, it's either a close Emoji or just text
                else {
                    is_last_char_rule(&mut stack, &mut buffer, &InlineId::Emoji, 1);
                    break;
                }
            }
            '`' => {
                // Step to next 2 characters to verify for Emoji
                content_chars.next();
                if let Some(post_char) = content_chars.peek() {
                    match post_char {
                        '`' => {
                            content_chars.next();
                            if let Some(nxt_char) = content_chars.peek() {
                                match nxt_char {
                                    punctuation if punctuations.contains(punctuation) | punctuation.is_whitespace() => nxt_is_whitespace_rule(&mut stack, &mut content_chars, &mut buffer, InlineId::Code, 2),
                                    _ =>  {
                                        nxt_non_whitespace_rule(&mut stack, &mut buffer, &InlineId::Code, 2, false);
                                    }
                                }
                            }
                            else {
                                is_last_char_rule(&mut stack, &mut buffer, &InlineId::Code, 2);
                                break;
                            }
                        }
                        _ => {
                            stack.push(InlineToken::Text("``".to_owned()));
                            content_chars.next();
                        }
                    }
                }
                else {
                    stack.push(InlineToken::Text("`".to_owned()));
                    break;
                }
            }
            _ => {
                buffer.push(*nxt_char);
                content_chars.next();
                if let None = content_chars.peek() {
                    stack.push(InlineToken::Text(buffer.clone()));
                    buffer.clear();
                }
            }
        }
    }

    for index in concerns {
        if let Some(token) = stack.get_mut(index-1) {
            match token.get_pos() {
                InlineTokenPos::Open | InlineTokenPos::None => {
                    match token.get_id() {
                        InlineId::Code => stack.push(InlineToken::Text("``".to_owned())),
                        InlineId::Bold => stack.push(InlineToken::Text("*".to_owned())),
                        InlineId::Strike => stack.push(InlineToken::Text("~".to_owned())),
                        InlineId::Italic => stack.push(InlineToken::Text("_".to_owned())),
                        InlineId::Caption => stack.push(InlineToken::Text("**".to_owned())),
                        InlineId::Underline => stack.push(InlineToken::Text("__".to_owned())),
                        InlineId::Emoji => stack.push(InlineToken::Text(":".to_owned())),
                        InlineId::Highlight => stack.push(InlineToken::Text("==".to_owned())),
                        _ => {}
                    }
                    stack.swap_remove(index-1);
                },
                InlineTokenPos::Close => {},
            }
        }
    }
    stack
}
