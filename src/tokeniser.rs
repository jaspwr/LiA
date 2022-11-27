use crate::utils::*;

#[derive(PartialEq)]
enum CharGroup {
    Whitespace,
    String,
    Symbol,
    Bracket,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl Location {
    pub fn stringify(&self) -> String {
        format!("{}:{}", self.line, self.column)
    }
}

fn classify_char (c: &char) -> CharGroup {
    match c {
        ' ' | '\t' | '\n' | '\r' | '\x0C' | '\x0B' => CharGroup::Whitespace,
        '(' | ')' | '{' | '}' | '[' | ']' => CharGroup::Bracket,
        '=' | '>' | ',' | '#' | '*' | ':' | ';' | '+' | '-' | '/' | '$' => CharGroup::Symbol,
        _ => CharGroup::String,
    }
}

pub type TokenList = Vec<Token>;

pub fn to_tokens (input_lia: String) -> TokenList {
    let mut ret = Vec::<Token>::new();
    let mut current_token = String::new();
    let mut pre_char_group = CharGroup::Whitespace;
    let mut first_of_line = true;
    let mut line: usize = 1;
    let mut column: usize = 1;
    let mut pre_c = ' ';
    input_lia.chars().for_each(|c| {
        if c == '\r' { return; }
        if c == '\n' || c == ';' { first_of_line = true;
            line += 1; column = 0;
            let token = parse_token(&current_token, first_of_line, Location { line, column });
            ret.push(token);
            current_token.clear();
            ret.push(Token::Newline); 
            return; 
        }
        let char_group = classify_char(&c);
        if (char_group != pre_char_group || pre_char_group == CharGroup::Bracket) && pre_c != '\\' {
            if !current_token.is_empty() {
                let token = parse_token(&current_token, first_of_line, Location { line, column });
                match token {
                    Token::Whitespace(_) => {},
                    _ => { first_of_line = false; }
                };
                ret.push(token);
                current_token.clear();
            }
        }
        current_token.push(c);
        column += 1;
        pre_char_group = char_group;
        pre_c = c;
    });
    let token = parse_token(&current_token, first_of_line, Location { line, column });
    ret.push(token);
    ret.push(Token::Newline);
    ret
}

#[derive(Debug, Clone)]
pub enum Token {
    TexCommand(String, Location),
    LiaVariable(String, Location),
    LiaKeyword(String, Location),
    LiaMarkDown(String, Location),
    Newline,
    Whitespace(String),
    Nothing(String, Location)
}

impl Token {
    pub fn get_location(&self) -> Location {
        match self {
            Token::TexCommand(_, loc) => *loc,
            Token::LiaVariable(_, loc) => *loc,
            Token::LiaKeyword(_, loc) => *loc,
            Token::LiaMarkDown(_, loc) => *loc,
            Token::Newline => Location::default(),
            Token::Whitespace(_) => Location::default(),
            Token::Nothing(_, loc) => *loc,
        }
    }
}

fn parse_token (token: &String, begins_line: bool, location: Location) -> Token {
    let last = match token.chars().last() {
        Some(c) => { c },
        None => { ' ' }
    };
    if begins_line {
        if token.starts_with("#") {
            return Token::LiaMarkDown(token.clone(), location);
        } else if token == "*" {
            return Token::LiaMarkDown(token.clone(), location);
        }
    }
    if token.starts_with('\\') {
        Token::TexCommand(token.clone(), location)
    } else if token.starts_with('@') {
        Token::LiaVariable(token.clone(), location)
    } else if is_whitespace(last) {
        Token::Whitespace(token.clone())
    } else if token.as_str() == "env" {
        Token::LiaKeyword(token.clone(), location)
    } else if token.as_str() == "use" && begins_line {
        Token::LiaKeyword(token.clone(), location)
    } else {
        Token::Nothing(token.clone(), location)
    }
}