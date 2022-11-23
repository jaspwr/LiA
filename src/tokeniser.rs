use crate::utils::*;

#[derive(PartialEq)]
enum CharGroup {
    Whitespace,
    String,
    Symbol,
    Bracket,
}

fn classify_char (c: &char) -> CharGroup {
    match c {
        ' ' | '\t' | '\n' | '\r' | '\x0C' | '\x0B' => CharGroup::Whitespace,
        '(' | ')' | '{' | '}' | '[' | ']' => CharGroup::Bracket,
        '=' | '>' | ',' | '#' | '*' | ':' | ';' => CharGroup::Symbol,
        _ => CharGroup::String,
    }
}

pub type TokenList = Vec<Token>;

pub fn to_tokens (input_lia: String) -> TokenList {
    let mut ret = Vec::<Token>::new();
    let mut current_token = String::new();
    let mut pre_char_group = CharGroup::Whitespace;
    let mut first_of_line = true;
    input_lia.chars().for_each(|c| {
        if c == '\n' || c == ';' || c == '\r' { first_of_line = true; 
            let token = parse_token(&current_token, first_of_line);
            ret.push(token);
            current_token.clear();
            ret.push(Token::Newline); 
            return; 
        }
        let char_group = classify_char(&c);
        if char_group != pre_char_group || pre_char_group == CharGroup::Bracket {
            if !current_token.is_empty() {
                let token = parse_token(&current_token, first_of_line);
                match token {
                    Token::Whitespace(_) => {},
                    _ => { first_of_line = false; }
                };
                ret.push(token);
                current_token.clear();
            }
        }
        current_token.push(c);
        pre_char_group = char_group;
    });
    ret.push(Token::Newline);
    ret
}

#[derive(Debug, Clone)]
pub enum Token {
    TexCommand(String),
    LiaVariable(String),
    LiaKeyword(String),
    LiaMarkDown(String),
    Newline,
    Whitespace(String),
    Nothing(String)
}

fn parse_token (token: &String, begins_line: bool) -> Token {
    let last = match token.chars().last() {
        Some(c) => { c },
        None => { ' ' }
    };
    if begins_line {
        if token.starts_with("#") {
            return Token::LiaMarkDown(token.clone());
        } else if token == "*" {
            return Token::LiaMarkDown(token.clone());
        }
    }
    let lia_keywords = vec!["use", "env"];
    if token.starts_with('\\') {
        Token::TexCommand(token.clone())
    } else if token.starts_with('@') {
        Token::LiaVariable(token.clone())
    } else if is_whitespace(last) {
        Token::Whitespace(token.clone())
    } else if lia_keywords.contains(&token.as_str()) {
        Token::LiaKeyword(token.clone())
    } else {
        Token::Nothing(token.clone())
    }
}