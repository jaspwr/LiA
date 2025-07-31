use std::rc::Rc;

use crate::bracket_depth::BrackDepths;
use crate::document::*;
use crate::parse::*;
use crate::token::*;
use crate::utils::untokenise;

#[derive(Default)]
pub struct InlineCode {
    start: Option<usize>,
}

impl NodeParser for InlineCode {
    fn is_opener(
        &mut self,
        tokens: &[Token],
        cursor: usize,
        identation: i32,
        other_doc_locations: &mut CompilerGlobals,
    ) -> bool {
        let token = &tokens[cursor];

        if let Token::Misc(ref t, _) = token {
            self.start = Some(cursor);
            return t == "`";
        }

        false
    }

    fn is_closer(&mut self, tokens: &[Token], cursor: usize, bracket_depths: &BrackDepths) -> bool {
        let token = &tokens[cursor];

        if self.start == Some(cursor) {
            return false;
        }

        if let Token::Misc(ref t, _) = token {
            return t == "`";
        }

        false
    }

    fn parse(
        &mut self,
        tokens: &[Token],
        range_start: usize,
        range_end: usize,
        indentation_type: Option<IndentationType>,
        other_doc_locations: &mut CompilerGlobals,
    ) -> ParseResult {
        let tokens = &tokens[range_start+1..range_end];

        Ok((
            vec![Rc::new(TexCommand {
                command: "texttt".to_string(),
                args: vec![Arg {
                    arg: vec![Rc::new(Text {
                        text: untokenise(tokens),
                    })],
                    arg_type: ArgType::Curly,
                }],
            })],
            DocSection::Document,
        ))
    }
}
