use std::rc::Rc;

use crate::{tokeniser::{Token, TokenList}, hierachy_construction::{BrackDepths, NodeParser, node_list}, hierarchy::{Node, TexCommand, Arg, ArgType}, utils::parse_args};

#[derive(Default)]
pub struct LiaVariableParser {}

impl NodeParser for LiaVariableParser {
    fn is_target(&self, token: &Token) -> bool {
        match token {
            Token::LiaVariable(_) => { true },
            _ => { false }
        }
    }

    fn is_closer(&self, token: &Token, next_token: &Token, next_token_no_white_space: &Token, bracket_depths: &BrackDepths) -> bool {
        // if bracket_depths.round != 0 || bracket_depths.curly != 0 { return false; }
        // match token {
        //     Token::LiaVariable(_) => {
        //         !(match next_token {
        //             Token::Nothing(t) => { t == "(" },
        //             _ => { false }
        //         } || match next_token_no_white_space {
        //             Token::Nothing(t) => { t == "=" },
        //             _ => { false }
        //         })
        //     },
        //     _ => {
        //         match token {
        //             Token::Newline => { true },
        //             _ => { false }
        //         }
        //     }
        // }
        true
    }

    fn parse (&self, tokens: TokenList) -> Vec<Rc<dyn Node>> {
        let command = match &tokens[0] {
            Token::LiaVariable(command) => { &command[1..] },
            _ => { todo!() }
        }.to_string();
        vec!{Rc::new( TexCommand {
            command,
            args: vec![]
        })}
    }
}