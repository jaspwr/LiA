use std::rc::Rc;

use crate::{tokeniser::{Token, TokenList}, hierachy_construction::{BrackDepths, NodeParser, node_list, IndentationType, ParseResult, DocSection, OtherDocLocations}, hierarchy::TexEnvironment};

#[derive(Default)]
pub struct LiaEnvParser {}

#[allow(unused)]
impl NodeParser for LiaEnvParser {
    fn is_target(&mut self, token: &Token, identation: i32) -> bool {
        match token {
            Token::LiaKeyword(k, _) => { k == "env" },
            _ => { false }
        }
    }

    fn is_closer(&mut self, token: &Token, next_token: &Token, next_token_no_white_space: &Token, bracket_depths: &BrackDepths) -> bool {
        match token {
            Token::Nothing(t, _) => { t == "}" && bracket_depths.curly == 0 },
            _ => { false }
        }
    }

    fn parse (&mut self, tokens: TokenList, indentation_type: Option<IndentationType>, other_doc_locations: &mut OtherDocLocations) -> ParseResult {
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
            Token::Nothing(command, _) => { command },
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
        let children = node_list(tokens, command_pos, len-1, other_doc_locations)?;
        if command == "document" {
            // LiA adds the document macro implicitly, ignore existing document macro.
            return Ok((children, DocSection::Imports))
        }
        Ok((vec!{Rc::new( TexEnvironment {
            name: command,
            args: vec![],
            children
        })}, DocSection::Document))
    }
}