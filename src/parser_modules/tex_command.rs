use std::rc::Rc;

use crate::{tokeniser::{Token, TokenList}, hierachy_construction::{BrackDepths, NodeParser, IndentationType, ParseResult}, hierarchy::TexCommand, utils::parse_args};

#[derive(Default)]
pub struct TexCommandParser {}

#[allow(unused)]
impl NodeParser for TexCommandParser {
    fn is_target(&mut self, token: &Token, identation: i32) -> bool {
        match token {
            Token::TexCommand(_, _) => { true },
            _ => { false }
        }
    }

    fn is_closer(&mut self, token: &Token, next_token: &Token, next_token_no_white_space: &Token, bracket_depths: &BrackDepths) -> bool {
        bracket_depths.curly == 0 && bracket_depths.square == 0
        && match next_token {
            Token::Nothing(t, _) => { t != "{" && t != "[" },
            _ => { true }
        }
    }

    fn parse (&mut self, tokens: TokenList, indentation_type: Option<IndentationType>) -> ParseResult {
        let command = match &tokens[0] {
            Token::TexCommand(command, _) => { &command[1..] },
            _ => { todo!() }
        }.to_string();
        Ok(vec!{Rc::new( TexCommand {
            command,
            args: parse_args(&tokens, 1, tokens.len())?
        })})
    }
}