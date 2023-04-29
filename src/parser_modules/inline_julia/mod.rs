use std::rc::Rc;

use crate::hierarchy::{ DocSection, Text };
use crate::bracket_depth::BrackDepths;
use crate::utils::format_error_string;
use crate::tokeniser::TokenList;
use crate::token::*;
use crate::hierachy_construction::{ NodeParser, IndentationType, ParseResult, CompilerGlobals };

#[derive(Default)]
pub struct InlineJulia {
    curly_depth: i32,
}

mod executor;
use executor::*;

#[allow(unused)]
impl NodeParser for InlineJulia {
    fn is_opener(
        &mut self,
        token: &Token,
        identation: i32,
        other_doc_locations: &mut CompilerGlobals
    ) -> bool {
        self.curly_depth = -1;
        match token {
            Token::LiaKeyword(k, _) => { k == "jl" }
            _ => { false }
        }
    }

    fn is_closer(
        &mut self,
        token: &Token,
        next_token: &Token,
        next_token_no_white_space: &Token,
        bracket_depths: &BrackDepths
    ) -> bool {
        if self.curly_depth == -1 {
            self.curly_depth = bracket_depths.curly;
        }
        match token {
            Token::Misc(t, _) => { t == "}" && bracket_depths.curly == self.curly_depth }
            _ => { false }
        }
    }

    fn parse(
        &mut self,
        tokens: TokenList,
        indentation_type: Option<IndentationType>,
        other_doc_locations: &mut CompilerGlobals
    ) -> ParseResult {
        let mut open_pos = 1;
        let len = tokens.len();
        while open_pos < len {
            if let Token::Whitespace(_) = tokens[open_pos] {
                open_pos += 1;
            } else if let Token::Misc(t, loc) = &tokens[open_pos] {
                if t == "{" {
                    break;
                } else {
                    return format_error_string(
                        format!("Unexpected token \"{}\" in inline Julia statement.", t),
                        *loc
                    );
                }
            } else {
                return format_error_string(
                    "Unexpected token in inline Julia statement.".to_string(),
                    tokens[open_pos].get_location()
                );
            }
        }
        open_pos += 1;


        let mut jl_code = String::new();
        for i in open_pos..len-1 {
            jl_code += &tokens[i].stringify();
        }
        print!("\n");

        let text_node = Rc::new(Text {
            text: execute(jl_code, other_doc_locations)?,
        });

        Ok((
            vec![
                text_node.clone()
            ],
            DocSection::Document,
        ))
    }
}