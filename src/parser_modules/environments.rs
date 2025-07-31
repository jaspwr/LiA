use std::rc::Rc;

use crate::bracket_depth::BrackDepths;
use crate::document::{DocSection, TexEnvironment, Text};
use crate::parse::{node_list, CompilerGlobals, IndentationType, NodeParser, ParseResult};
use crate::token::*;
use crate::utils::format_error_string;

#[derive(Default)]
pub struct LiaEnvParser {
    curly_depth: i32,
}

#[allow(unused)]
impl NodeParser for LiaEnvParser {
    fn is_opener(
        &mut self,
        tokens: &[Token],
        cursor: usize,
        identation: i32,
        other_doc_locations: &mut CompilerGlobals,
    ) -> bool {
        let token = &tokens[cursor];

        self.curly_depth = -1;
        match token {
            Token::LiaKeyword(k, _) => k == "env",
            _ => false,
        }
    }

    fn is_closer(&mut self, tokens: &[Token], cursor: usize, bracket_depths: &BrackDepths) -> bool {
        let token = &tokens[cursor];

        if self.curly_depth == -1 {
            self.curly_depth = bracket_depths.curly;
        }

        // println!("       {:?} {}", token, bracket_depths.curly);

        match token {
            Token::Misc(t, _) => t == "}" && bracket_depths.curly == self.curly_depth,
            _ => false,
        }
    }

    fn parse(
        &mut self,
        tokens: &[Token],
        range_start: usize,
        range_end: usize,
        indentation_type: Option<IndentationType>,
        other_doc_locations: &mut CompilerGlobals,
    ) -> ParseResult {
        let tokens = &tokens[range_start..=range_end];

        // println!("{:?}", tokens);

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

        if command == "verbatim" {
            return Ok((
                vec![Rc::new(TexEnvironment {
                    name: command,
                    args: vec![],
                    children: vec![Rc::new(Text {
                        text: tokens
                            .iter()
                            .skip(command_pos)
                            .take(len - 2)
                            .map(Token::stringify)
                            .collect::<Vec<_>>()
                            .join(""),
                    })],
                })],
                DocSection::Document,
            ));
        }

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
