use std::rc::Rc;

use crate::{hierachy_construction::*, tokeniser::{Token, TokenList}, hierarchy::*};

#[derive(Default)]
pub struct LiaMarkDownSections {}

impl NodeParser for LiaMarkDownSections {
    fn is_target(&mut self, token: &Token, identation: i32) -> bool {
        match token {
            Token::LiaMarkDown(text) => { 
                //println!("LiaMarkDown: {}", text);
                text.starts_with("#")
            },
            _ => { false }
        }
    }

    fn is_closer(&mut self, token: &Token, next_token: &Token, next_token_no_white_space: &Token, bracket_depths: &BrackDepths) -> bool {
        match token {
            Token::Newline => { bracket_depths.curly == 0 },
            _ => { false }
        }
    }

    fn parse (&mut self, tokens: TokenList, indentation_type: Option<IndentationType>) -> Vec<Rc<dyn Node>> {
        println!("meow {:?}", tokens[0]);
        let command = match &tokens[0] {
            Token::LiaMarkDown(hash) => { 
                match hash.as_str() {
                    "#" => { "section" },
                    "##" => { "subsection" },
                    "###" => { "subsubsection" },
                    "#*" => { "section*" },
                    "##*" => { "subsection*" },
                    "###*" => { "subsubsection*" },
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
                        node_list(tokens, start, len-1)
                    },
                    arg_type: ArgType::Curly
                }]
        })}
    }
}