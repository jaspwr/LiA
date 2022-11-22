use crate::utils::*;

#[derive(Debug)]
pub struct token {
    string: String,
    id: u32
}

type token_index = usize;

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

pub fn to_tokens (input_lia: String) -> Vec<token_index> {
    let mut ret = Vec::<token_index>::new();
    let mut current_token = String::new();
    let mut pre_char_group = CharGroup::Whitespace;
    let mut first_of_line = true;
    input_lia.chars().for_each(|c| {
        if c == '\n' || c == ';' { first_of_line = true; println!("Token: {:?}", Token::Newline); }
        let char_group = classify_char(&c);
        if char_group != pre_char_group || pre_char_group == CharGroup::Bracket {
            if !current_token.is_empty() {
                let token = parse_token(&current_token, first_of_line);
                first_of_line = false;
                println!("Token: {:?}", token);
                current_token.clear();
            }
        }
        current_token.push(c);
        pre_char_group = char_group;
    });
    print!("\n");
    ret
}

#[derive(Debug)]
enum Token {
    TexCommand(String),
    TexBracket(String),
    LiaVariable(String),
    LiaKeyword(String),
    LiaMarkDown(String),
    Newline,
    Whitespace(String),
    Nothing(String)
}

fn parse_token (token: &String, begins_line: bool) -> Token {
    if begins_line {
        if token == "#" {
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
    } else if is_whitespace(token.chars().last().unwrap()) {
        Token::Whitespace(token.clone())
    } else if lia_keywords.contains(&token.as_str()) {
        Token::LiaKeyword(token.clone())
    } else {
        Token::Nothing(token.clone())
    }
}

// fn construct_heirarchy (tokens: Vec<token_index>) -> Vec<token_index> {
//     let mut ret = Vec::<token_index>::new();
//     ret
// }