use std::{fs::File, io::Read};

use crate::{tokeniser::{TokenList, Token}, hierarchy::{ArgList, ArgType, Arg}, hierachy_construction::{BrackDepths, node_list}};

pub fn load_utf8_file (path: String) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn is_whitespace (char: char) -> bool {
    char == ' ' || char == '\t' || char == '\n' || char == '\r' || char == '\x0C' || char == '\x0B'
}

pub fn parse_args (tokens: &TokenList, start: usize, end: usize) -> ArgList {
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
                    arg: node_list(tokens.clone(), arg_start, i)
                });
                arg_type = None;
            }
        }
    }
    ret
}

pub fn delta_bracket_depth (token: &Token) -> BrackDepths {
    let mut bracket_depths = BrackDepths::default();
    if let Token::Nothing(str) = token {
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