use std::{rc::Rc, str::EncodeUtf16};

use crate::{hierachy_construction::*, tokeniser::{Token, TokenList}, hierarchy::*, utils::{delta_bracket_depth, parse_args}};

#[derive(Default)]
pub struct LiaUseParser {}

impl NodeParser for LiaUseParser {
    fn is_target(&self, token: &Token) -> bool {
        match token {
            Token::LiaKeyword(k) => { k == "use" },
            _ => { false }
        }
    }

    fn is_closer(&self, token: &Token, next_token: &Token, bracket_depths: &BrackDepths) -> bool {
        match token {
            Token::Newline => { bracket_depths.curly == 0 },
            _ => { false }
        }
    }

    fn parse (&self, tokens: TokenList) -> Vec<Rc<dyn Node>> {
        let mut imports: Vec<ArgList> = Vec::new();

        let len = tokens.len();
        
        imports.push(parse_to_args(tokens.clone(), 1));
        let mut start = 1;
        while start < len {
            match &tokens[start] {
                Token::Nothing(sym) => {
                    if sym == "," {
                        imports.push(parse_to_args(tokens.clone(), start+1));
                    }
                },
                _ => {}
            };
            start += 1; 
        }


        imports.into_iter().map(|args| {
            Rc::new( TexCommand {
                command: "usepackage".to_string(),
                args
            }) as Rc<dyn Node>
        }).collect()
    }
}

fn parse_to_args (tokens: TokenList, start: usize) -> ArgList {
    let len = tokens.len();
    let mut start = start;
    
    while start < len {
        if let Token::Whitespace(_) = tokens[start] { start += 1; } else { break; }
    }
    let mut bracket_depth: BrackDepths = BrackDepths { curly: 0, square: 0, round: 0 };
    let mut end = start;
    while end < len {
        bracket_depth += delta_bracket_depth(&tokens[end]);
        if bracket_depth.curly == 0 && bracket_depth.square == 0 {
            if end + 1 > len { break; }
            if let Token::Nothing(s) = &tokens[end+1] {
                if s == "[" || s == "{" {
                    end += 1;
                    continue;
                } else {
                    end += 1;
                    break;
                }
            } else {
                end += 1;
                break;
            }
        } else {
            end += 1;
        }
    }
    

    let mut args = parse_args(&tokens, start, end);

    if end + 1 > len {
        panic!("No package name");
    }
    if args.len() == 0 {
        end -= 1;
    }
    args.push(Arg { arg: node_list(tokens, end, end+1), arg_type: ArgType::Curly });
    args
}