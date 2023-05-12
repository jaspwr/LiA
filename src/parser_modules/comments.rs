use std::rc::Rc;

use crate::bracket_depth::BrackDepths;
use crate::hierarchy::*;
use crate::hierarchy_construction::*;
use crate::token::*;
use crate::tokeniser::TokenList;

#[derive(Default)]
pub struct Comment {}

#[allow(unused)]
impl NodeParser for Comment {
    fn is_opener(
        &mut self,
        token: &Token,
        identation: i32,
        other_doc_locations: &mut CompilerGlobals,
    ) -> bool {
        match token {
            Token::Misc(text, _) => text == "%",
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
        match token {
            Token::Newline => true,
            _ => false,
        }
    }

    fn parse(
        &mut self,
        tokens: TokenList,
        indentation_type: Option<IndentationType>,
        other_doc_locations: &mut CompilerGlobals,
    ) -> ParseResult {
        Ok((
            vec![Rc::new(Text {
                text: "\n".to_string(),
            })],
            DocSection::Document,
        ))
    }
}
