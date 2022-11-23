use std::rc::Rc;

use crate::{tokeniser::{Token, TokenList}, hierachy_construction::{BrackDepths, NodeParser, node_list}, hierarchy::{Node, TexCommand, Arg, ArgType}, utils::parse_args};

#[derive(Default)]
pub struct TexCommandParser {}

impl NodeParser for TexCommandParser {
    fn is_target(&self, token: &Token) -> bool {
        match token {
            Token::TexCommand(_) => { true },
            _ => { false }
        }
    }

    fn is_closer(&self, token: &Token, next_token: &Token, next_token_no_white_space: &Token, bracket_depths: &BrackDepths) -> bool {
        bracket_depths.curly == 0 && bracket_depths.square == 0
        && match next_token {
            Token::Nothing(t) => { t != "{" && t != "[" },
            _ => { true }
        }
    }

    fn parse (&self, tokens: TokenList) -> Vec<Rc<dyn Node>> {
        let command = match &tokens[0] {
            Token::TexCommand(command) => { &command[1..] },
            _ => { todo!() }
        }.to_string();
        vec!{Rc::new( TexCommand {
            command,
            args: parse_args(&tokens, 1, tokens.len())
        })}
    }
}