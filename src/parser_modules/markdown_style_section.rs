use std::rc::Rc;

use crate::{hierachy_construction::*, tokeniser::{Token, TokenList}, hierarchy::*};

#[derive(Default)]
pub struct LiaMarkDownSections {}

impl NodeParser for LiaMarkDownSections {
    fn is_target(&self, token: &Token) -> bool {
        match token {
            Token::LiaMarkDown(text) => { 
                println!("LiaMarkDown: {}", text);
                if text == "###" {
                    true
                } else if text == "##" {
                    true
                } else if text == "#" {
                    true
                } else {
                    false
                }    
            },
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
        println!("meow {:?}", tokens[0]);
        let command = match &tokens[0] {
            Token::LiaMarkDown(hash) => { 
                match hash.as_str() {
                    "#" => { "section" },
                    "##" => { "subsection" },
                    "###" => { "subsubsection" },
                    _ => { todo!() }
                }
            },
            _ => { todo!() }
        }.to_string();
        vec!{Rc::new( TexCommand {
            command,
            args: vec![Arg {
                    arg: {
                        let len = tokens.len();
                        let mut start = 1;
                        while start < len {
                            if let Token::Whitespace(_) = tokens[start] {
                                start += 1;
                            } else {
                                break;
                            }
                        }
                        node_list(tokens, start, len)
                    },
                    arg_type: ArgType::Curly
                }]
        })}
    }
}