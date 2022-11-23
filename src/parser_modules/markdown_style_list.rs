use std::rc::Rc;

use crate::{tokeniser::{Token, TokenList}, hierachy_construction::{BrackDepths, NodeParser, node_list}, hierarchy::{Node, TexCommand, Arg, ArgType, TexEnvironment, NodeList}, utils::parse_args};

#[derive(Default)]
pub struct LiaMardownListParser {}

impl NodeParser for LiaMardownListParser {
    fn is_target(&self, token: &Token) -> bool {
        match token {
            Token::LiaMarkDown(text) => { text == "*" },
            _ => { false }
        }
    }

    fn is_closer(&self, token: &Token, next_token: &Token, next_token_no_white_space: &Token, bracket_depths: &BrackDepths) -> bool {
        bracket_depths.curly == 0
        && match token {
            Token::Newline => { 
                match next_token_no_white_space {
                    Token::LiaMarkDown(text) => { text != "*" },
                    _ => { true }
                }
            },
            _ => { false }
        }
    }

    fn parse (&self, tokens: TokenList) -> Vec<Rc<dyn Node>> {
        // let command = match &tokens[0] {
        //     Token::LiaVariable(command) => { &command[1..] },
        //     _ => { todo!() }
        // }.to_string();
        let t: TokenList = tokens.into_iter().map(|token| -> Token {
            match &token {
                Token::LiaMarkDown(md) => { 
                    if md == "*" {
                        Token::TexCommand("\\item".to_string())
                    } else {
                        token
                    }
                },
                _ => { token }
            }
        }).collect();
        vec!{Rc::new( TexEnvironment {
            name: "itemize".to_string(),
            args: vec![],
            children: node_list(t.clone(), 0, t.len())
        })}
    }
}