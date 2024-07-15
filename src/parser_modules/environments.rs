use std::rc::Rc;

use crate::bracket_depth::BrackDepths;
use crate::document::{DocSection, TexEnvironment};
use crate::parse::{
    node_list, CompilerGlobals, IndentationType, NodeParser, ParseResult,
};
use crate::token::*;
use crate::tokenize::TokenList;
use crate::utils::format_error_string;

#[derive(Default)]
pub struct LiaEnvParser {
    curly_depth: i32,
}

#[allow(unused)]
impl NodeParser for LiaEnvParser {
    fn is_opener(
        &mut self,
        token: &Token,
        identation: i32,
        other_doc_locations: &mut CompilerGlobals,
    ) -> bool {
        self.curly_depth = -1;
        match token {
            Token::LiaKeyword(k, _) => k == "env",
            _ => false,
        }
    }

    fn is_closer(
        &mut self,
        token: &Token,
        next_token: &Token,
        next_token_no_white_space: &Token,
        bracket_depths: &BrackDepths,
    ) -> bool {
        if self.curly_depth == -1 {
            self.curly_depth = bracket_depths.curly;
        }
        match token {
            Token::Misc(t, _) => t == "}" && bracket_depths.curly == self.curly_depth,
            _ => false,
        }
    }

    fn parse(
        &mut self,
        tokens: TokenList,
        indentation_type: Option<IndentationType>,
        other_doc_locations: &mut CompilerGlobals,
    ) -> ParseResult {
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
            Token::Misc(command, _) => command,
            _ => {
                return format_error_string(
                    "Unexpected token in environment statement. Aborted".to_string(),
                    tokens[command_pos].get_location(),
                );
            }
        }
        .to_string();
        command_pos += 1;
        while command_pos < len {
            if let Token::Whitespace(_) = &tokens[command_pos] {
                command_pos += 1;
            } else if let Token::Misc(t, loc) = &tokens[command_pos] {
                if t == "{" {
                    break;
                } else {
                    return format_error_string(
                        "Unexpected token in environment statement.".to_string(),
                        *loc,
                    );
                }
            } else {
                return Err("Unexpected token in environment statement.".to_string());
            }
        }
        command_pos += 1;
        let children = node_list(tokens, command_pos, len - 1, other_doc_locations)?;
        if command == "document" {
            // LiA adds the document macro implicitly, ignore existing document macro.
            return Ok((children, DocSection::Imports));
        }
        Ok((
            vec![Rc::new(TexEnvironment {
                name: command,
                args: vec![],
                children,
            })],
            DocSection::Document,
        ))
    }
}
