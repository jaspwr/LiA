use std::rc::Rc;

use crate::bracket_depth::BrackDepths;
use crate::document::*;
use crate::parse::*;
use crate::token::*;

enum BOrI {
    Bold,
    Italic,
}

#[derive(Default)]
pub struct BoldItalic {
    start: Option<usize>,
    end: bool,
    b_or_i: Option<BOrI>,
}

impl NodeParser for BoldItalic {
    fn is_opener(
        &mut self,
        tokens: &[Token],
        cursor: usize,
        _identation: i32,
        _other_doc_locations: &mut CompilerGlobals,
    ) -> bool {
        let token = &tokens[cursor];

        self.b_or_i = None;
        if self.end {
            self.end = false;
            return false;
        }

        if let Token::Misc(text, _) = token {
            if text.starts_with("**") {
                self.b_or_i = Some(BOrI::Italic);
                if text.starts_with("***") {
                    self.b_or_i = Some(BOrI::Bold);
                }

                self.start = Some(cursor);

                return true;
            }
        }

        false
    }

    fn is_closer(
        &mut self,
        tokens: &[Token],
        cursor: usize,
        _bracket_depths: &BrackDepths,
        _start_bracket_depths: &BrackDepths,
    ) -> bool {
        if self.start == Some(cursor) {
            return false;
        }

        let token = &tokens[cursor];

        match token {
            Token::Misc(t, _) => t.starts_with('*'),
            Token::Newline => true,
            _ => false,
        }
    }

    fn parse(
        &mut self,
        tokens: &[Token],
        range_start: usize,
        range_end: usize,
        _indentation_type: Option<IndentationType>,
        other_doc_locations: &mut CompilerGlobals,
    ) -> ParseResult {
        let tokens = &tokens[range_start..=range_end];

        self.end = true;
        let len = tokens.len();
        Ok((
            vec![Rc::new(TexCommand {
                command: if let Some(BOrI::Bold) = self.b_or_i {
                    "textbf".to_string()
                } else {
                    "textit".to_string()
                },
                args: vec![Arg {
                    arg: { node_list(tokens, 1, len - 1, other_doc_locations)? },
                    arg_type: ArgType::Curly,
                }],
            })],
            DocSection::Document,
        ))
    }
}
