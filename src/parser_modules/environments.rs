use std::rc::Rc;

use crate::{tokeniser::{Token, TokenList}, hierachy_construction::{BrackDepths, NodeParser, node_list}, hierarchy::{Node, TexCommand, Arg, ArgType, TexEnvironment}};

#[derive(Default)]
pub struct LiaEnvParser {}

impl NodeParser for LiaEnvParser {
    fn is_target(&self, token: &Token) -> bool {
        match token {
            Token::LiaKeyword(k) => { k == "env" },
            _ => { false }
        }
    }

    fn is_closer(&self, token: &Token, next_token: &Token, bracket_depths: &BrackDepths) -> bool {
        println!("is_closer: {:?} {:?}", token, bracket_depths.curly );
        match token {
            Token::Nothing(t) => { t == "}" && bracket_depths.curly == 0 },
            _ => { false }
        }
    }

    fn parse (&self, tokens: TokenList) -> Vec<Rc<dyn Node>> {
        let mut command_pos = 1;
        let len = tokens.len();
        while command_pos < len {
            if let Token::Whitespace(_) = tokens[command_pos] {
                command_pos += 1;
            } else {
                break;
            }
        }
        let command = match &tokens[command_pos] {
            Token::Nothing(command) => { command },
            _ => { todo!() }
        }.to_string();
        command_pos += 1;
        while command_pos < len {
            if let Token::Whitespace(_) = tokens[command_pos] {
                command_pos += 1;
            } else {
                break;
            }
        }
        command_pos += 1;
        vec!{Rc::new( TexEnvironment {
            name: command,
            args: vec![],
            children: node_list(tokens, command_pos, len-1)
        })}
    }
}