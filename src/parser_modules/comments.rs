use std::rc::Rc;

use crate::bracket_depth::BrackDepths;
use crate::document::*;
use crate::parse::*;
use crate::token::*;

#[derive(Default)]
pub struct Comment {}

impl NodeParser for Comment {
    fn is_opener(
        &mut self,
        tokens: &[Token],
        cursor: usize,
        identation: i32,
        other_doc_locations: &mut CompilerGlobals,
    ) -> bool {
        let token = &tokens[cursor];

        match token {
            Token::Misc(text, _) => text == "%",
            _ => false,
        }
    }

    fn is_closer(&mut self, tokens: &[Token], cursor: usize, bracket_depths: &BrackDepths) -> bool {
        let token = &tokens[cursor];

        match token {
            Token::Newline => true,
            _ => false,
        }
    }

    fn parse(
        &mut self,
        _tokens: &[Token],
        _range_start: usize,
        _range_end: usize,
        _indentation_type: Option<IndentationType>,
        _other_doc_locations: &mut CompilerGlobals,
    ) -> ParseResult {
        Ok((
            vec![Rc::new(Text {
                text: "\n".to_string(),
            })],
            DocSection::Document,
        ))
    }
}
