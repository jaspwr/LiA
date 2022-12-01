use std::rc::Rc;

use crate::hierarchy::{TexEnvironment, DocSection};
use crate::bracket_depth::BrackDepths;
use crate::utils::format_error_string;
use crate::tokeniser::TokenList;
use crate::token::*;
use crate::hierachy_construction::{NodeParser, node_list, IndentationType, ParseResult, OtherDocLocations};

#[derive(Default)]
pub struct LiaEquation {}

#[allow(unused)]
impl NodeParser for LiaEquation {
    fn is_opener(&mut self, token: &Token, identation: i32) -> bool {
        match token {
            Token::Nothing(k, _) => { k == "eq" },
            _ => { false }
        }
    }

    fn is_closer(&mut self, token: &Token, next_token: &Token, next_token_no_white_space: &Token, bracket_depths: &BrackDepths) -> bool {
        match token {
            Token::Nothing(t, _) => { t == "}" && bracket_depths.curly == 0 },
            _ => { false }
        }
    }

    fn parse (&mut self, tokens: TokenList, indentation_type: Option<IndentationType>, 
        other_doc_locations: &mut OtherDocLocations) -> ParseResult {
            let mut asterisk = false;
            let mut open_pos = 1;
            let len = tokens.len();
            while open_pos < len {
                if let Token::Whitespace(_) = tokens[open_pos] {
                    open_pos += 1;
                } else if let Token::Nothing(t, loc) = &tokens[open_pos] {
                    if t == "*" {
                        asterisk = true;
                        open_pos += 1;
                    } else if t == "{" {
                        break;
                    } else {
                        return format_error_string(
                            format!{"Unexpected token \"{}\" in equation statement.", t}, 
                            *loc);
                    }
                } else {
                    return format_error_string(
                        "Unexpected token in equation statement.".to_string(), 
                        tokens[open_pos].get_location());
                }
            }
            
            let children = node_list(tokens, open_pos + 1, len - 1, other_doc_locations)?;

            Ok((vec!{Rc::new( TexEnvironment {
                name: if asterisk { "[".to_string() } else { "equation".to_string() },
                args: vec![],
                children
            })}, DocSection::Document))
    }
}