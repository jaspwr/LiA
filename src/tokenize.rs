use crate::{token::*, utils::*};

#[derive(PartialEq)]
enum CharGroup {
    Whitespace,
    String,
    Symbol,
    Bracket,
}

fn classify_char(c: &char) -> CharGroup {
    match c {
        ' ' | '\t' | '\n' | '\r' | '\x0C' | '\x0B' => CharGroup::Whitespace,
        '(' | ')' | '{' | '}' | '[' | ']' => CharGroup::Bracket,
        '=' | '>' | ',' | '#' | '*' | ':' | '%' | '<' | '~' | '!' | ';' | '+' | '-' | '/' | '^' | '`'
        | '_' | '$' => CharGroup::Symbol,
        _ => CharGroup::String,
    }
}

pub type TokenList<'a> = &'a [Token];

pub fn to_tokens(input_lia: String) -> Vec<Token> {
    let mut ret = Vec::<Token>::new();
    let mut current_token = String::new();
    let mut pre_char_group = CharGroup::Whitespace;
    let mut first_of_line = true;
    let mut line: usize = 1;
    let mut column: usize = 1;
    let mut start_of_token = Location::default();
    let mut pre_c = ' ';
    input_lia.chars().for_each(|c| {
        if c == '\r' {
            return;
        }
        if c == '\n' {
            line += 1;
            column = 1;
            let token = parse_token(&current_token, first_of_line, Location { line, column });
            ret.push(token);
            start_new_token(&mut start_of_token, line, column, &mut current_token);
            ret.push(Token::Newline);
            first_of_line = true;
            return;
        }
        let char_group = classify_char(&c);
        if (char_group != pre_char_group
            || pre_char_group == CharGroup::Bracket
            || c == '\\'
            || c == '@')
            && pre_c != '\\'
        {
            if !current_token.is_empty() {
                let token = parse_token(&current_token, first_of_line, start_of_token);
                match token {
                    Token::Whitespace(_) => {}
                    _ => {
                        first_of_line = false;
                    }
                };
                start_new_token(&mut start_of_token, line, column, &mut current_token);
                ret.push(token);
            }
        }
        current_token.push(c);
        column += 1;
        pre_char_group = char_group;
        pre_c = c;
    });
    let token = parse_token(&current_token, first_of_line, start_of_token);
    ret.push(token);
    ret.push(Token::Newline);
    ret
}

fn start_new_token(
    start_of_token: &mut Location,
    line: usize,
    column: usize,
    current_token: &mut String,
) {
    *start_of_token = Location { line, column };
    current_token.clear();
}

fn parse_token(token: &String, begins_line: bool, location: Location) -> Token {
    let last = match token.chars().last() {
        Some(c) => c,
        None => ' ',
    };
    if begins_line {
        if token.starts_with("#") {
            return Token::LiaMarkDown(token.clone(), location);
        } else if token == "*" {
            return Token::LiaMarkDown(token.clone(), location);
        }
    }
    if token.starts_with('\\') && token.chars().nth(1).map(|c| c != '@').unwrap_or(false) {
        Token::TexCommand(token.clone(), location)
    } else if token.starts_with('@') {
        Token::LiaVariable(token.clone(), location)
    } else if is_whitespace(last) {
        Token::Whitespace(token.clone())
    } else if token.as_str() == "env" {
        Token::LiaKeyword(token.clone(), location)
    } else if token.as_str() == "jl" {
        Token::LiaKeyword(token.clone(), location)
    } else if token.as_str() == "use" && begins_line {
        Token::LiaKeyword(token.clone(), location)
    } else {
        Token::Misc(token.clone(), location)
    }
}
