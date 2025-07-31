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
    start: bool,
    end: bool,
    b_or_i: Option<BOrI>,
}

impl NodeParser for BoldItalic {
    fn is_opener(
        &mut self,
        tokens: &[Token],
        cursor: usize,
        identation: i32,
        other_doc_locations: &mut CompilerGlobals,
    ) -> bool {
        let token = &tokens[cursor];

        self.start = true;
        self.b_or_i = None;
        if self.end {
            self.end = false;
            return false;
        }
        match token {
            Token::Misc(text, _) => {
                if text.starts_with("**") {
                    if text.starts_with("***") {
                        self.b_or_i = Some(BOrI::Bold);
                    } else {
                        self.b_or_i = Some(BOrI::Italic);
                    }
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn is_closer(&mut self, tokens: &[Token], cursor: usize, bracket_depths: &BrackDepths) -> bool {
        let token = &tokens[cursor];

        let mut ret = false;
        if !self.start {
            match token {
                Token::Misc(t, loc) => {
                    ret = t.starts_with('*');
                }
                Token::Newline => {
                    ret = true;
                }
                _ => {}
            }
        }
        self.start = false;
        ret
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
