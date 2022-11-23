use std::rc::Rc;

use crate::{hierachy_construction::*, tokeniser::{Token, TokenList}, hierarchy::*};

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
        vec!{Rc::new( TexCommand {
            command: "usepackage".to_string(),
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
        })
    }}
}