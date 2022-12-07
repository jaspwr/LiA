use std::{fs::{File, remove_file}, io::{Read, Write}};

use crate::{tokeniser::TokenList, token::*, hierarchy::{ArgList, ArgType, Arg}, bracket_depth::BrackDepths};
use crate::hierachy_construction::{node_list, IndentationType, ParseResult, CompilerGlobals};

pub fn load_utf8_file (path: String) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn write_utf8_file (path: String, contents: String) -> Result<(), std::io::Error> {
    let _ = remove_file(path.clone());
    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

pub fn is_whitespace (char: char) -> bool {
    char == ' ' || char == '\t' || char == '\n' || char == '\r' || char == '\x0C' || char == '\x0B'
}

pub fn is_bracket (char: char) -> bool {
    char == '(' || char == ')' || char == '{' || char == '}' || char == '[' || char == ']'
}

pub fn parse_args (tokens: &TokenList, start: usize, end: usize, other_doc_locations: &mut CompilerGlobals) -> Result<ArgList, String> {
    let mut ret: ArgList = Vec::new();
    let mut bracket_depths = BrackDepths::default();
    let mut arg_type: Option<ArgType> = None;
    let mut arg_start = 0;
    for i in start..end {

        let delta = delta_bracket_depth(&tokens[i]);
        if bracket_depths.curly == 0 && bracket_depths.square == 0 {
            if delta.curly == 1 {
                arg_type = Some(ArgType::Curly);
                arg_start = i + 1;
            } else if delta.square == 1 {
                arg_type = Some(ArgType::Square);
                arg_start = i + 1;
            }
        }
        bracket_depths += delta;
        if bracket_depths.curly == 0 && bracket_depths.square == 0 {
            if arg_type.is_some() {
                ret.push(Arg {
                    arg_type: arg_type.unwrap(),
                    arg: node_list(tokens.clone(), arg_start, i, other_doc_locations)?
                });
                arg_type = None;
            }
        }
    }
    Ok(ret)
}

pub fn delta_bracket_depth (token: &Token) -> BrackDepths {
    let mut bracket_depths = BrackDepths::default();
    if let Token::Nothing(str, _) = token {
        if str == "{" {
            bracket_depths.curly += 1;
        } else if str == "}" {
            bracket_depths.curly -= 1;
        } else if str == "[" {
            bracket_depths.square += 1;
        } else if str == "]" {
            bracket_depths.square -= 1;
        } else if str == "(" {
            bracket_depths.round += 1;
        } else if str == ")" {
            bracket_depths.round -= 1;
        }
    }
    bracket_depths
}

pub fn count_whitespace (tokens: &TokenList, start: usize) -> usize {
    //let mut count = 0;
    let mut count = 1;
    let len = tokens.len();
    while start + count < len {
        if let Token::Whitespace(_) = tokens[start + count] {
            count += 1;
        } else {
            if let Token::Newline = tokens[start + count] {
                count += 1;
            } else {
                break;
            }
        }
    }
    count
}

pub fn count_indentation (tokens: &TokenList, i: usize, indentation: &mut usize, indentation_type: &mut Option<IndentationType>) {
    if let Token::Newline = &tokens[if i > 0 { i - 1 } else { 0 }] {
        *indentation = 0;
        if let Token::Whitespace(whitespace) = &tokens[i] {
            if indentation_type.clone().is_none() {
                if whitespace.contains('\t') {
                    *indentation_type = Some(IndentationType::Tab);
                } else {
                    // TODO: Do this properly.
                    *indentation_type = Some(IndentationType::Space(if whitespace.len() % 4 == 0 { 4 } else { 2 }));
                }
            }
            match indentation_type.unwrap() {
                IndentationType::Tab => {
                    *indentation = whitespace.chars().filter(|c| *c == '\t').count();
                }
                IndentationType::Space(space_count) => {
                    *indentation = (whitespace.chars().filter(|c| *c == ' ').count() as f32 / space_count as f32).floor() as usize;
                }
            }
        }
    }
}

pub fn format_error_string (message: String, location: Location) -> ParseResult {
    Err(format!{"{} {}", location.stringify(), message})
}

pub fn hash_file(path: &String) -> String {
    let bytes: &[u8] = &std::fs::read(path).unwrap();
    let hash = sha256::digest(bytes);
    hash
}

pub fn indent(string: String, indentation: usize, indentation_type: IndentationType) -> String {
    let mut ret = String::new();
    for line in string.lines() {
        // Remove random single leading space.
        let mut line = line;
        match line.chars().nth(0) {
            Some(c) => {
                match line.chars().nth(1) {
                    Some(c2) => {
                        if c == ' ' && c2 != ' ' {
                            line = &line[1..];
                        }
                    }
                    None => {}
                }
            }, 
            None => {}
        }
        // Don't indent empty lines.
        if line.len() < 1 {
            ret.push_str("\n");
            continue;
        }
        for _ in 0..indentation {
            match indentation_type {
                IndentationType::Tab => {
                    ret.push('\t');
                }
                IndentationType::Space(space_count) => {
                    for _ in 0..space_count {
                        ret.push(' ');
                    }
                }
            }
        }
        ret.push_str(line);
        ret.push_str("\n");
    }
    ret
}

pub fn strip_tailing_whitespace_and_newlines (string: String) -> String {
    let mut white_space_count = 0;
    while is_whitespace(string[string.len() - white_space_count - 1..].chars().nth(0).unwrap()) {
        white_space_count += 1;
    }
    string[..string.len() - white_space_count].to_string()
}

pub fn untokenise (tokens: &TokenList) -> String {
    let mut ret = String::new();
    for token in tokens {
        ret.push_str(token.stringify().as_str());
    }
    ret
}

pub fn strip_all_whitespace (string: &str) -> String {
    let mut ret = String::new();
    for c in string.chars() {
        if !is_whitespace(c) {
            ret.push(c);
        }
    }
    ret
}