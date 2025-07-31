use std::rc::Rc;

use crate::bracket_depth::BrackDepths;
use crate::document::*;
use crate::parse::*;
use crate::token::*;
use crate::tokenize::TokenList;
use crate::utils::format_error_string;

#[derive(Default)]
pub struct LiaMarkDownSections {
}

impl NodeParser for LiaMarkDownSections {
    fn is_opener(
        &mut self,
        tokens: &[Token],
        cursor: usize,
        identation: i32,
        other_doc_locations: &mut CompilerGlobals,
    ) -> bool {
        let token = &tokens[cursor];

        match token {
            Token::LiaMarkDown(text, _) => text.starts_with("#"),
            _ => false,
        }
    }

    fn is_closer(&mut self, tokens: &[Token], cursor: usize, bracket_depths: &BrackDepths, start_bracket_depths: &BrackDepths) -> bool {
        let token = &tokens[cursor];
        match token {
            Token::Newline => bracket_depths.curly == start_bracket_depths.curly,
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

        let command = match &tokens[0] {
            Token::LiaMarkDown(hash, loc) => {
                match hash.as_str() {
                    "#" => { "section" },
                    "##" => { "subsection" },
                    "###" => { "subsubsection" },
                    "#*" => { "section*" },
                    "##*" => { "subsection*" },
                    "###*" => { "subsubsection*" },
                    _ => { return format_error_string(
                        format!{"Lines opened with '#' will automatically be assumed to be a section. \"{hash}\" is not a valid section command. If you don't want this to parse as a section, add a '\\' to escape it."},
                        *loc) }
                }
            },
            _ => { panic!("Should not be here.") }
        }.to_string();
        Ok((
            vec![
                Rc::new(TexCommand {
                    command,
                    args: vec![Arg {
                        arg: { rest_of_line(&tokens, other_doc_locations)? },
                        arg_type: ArgType::Curly,
                    }],
                }),
                Rc::new(Text {
                    text: "\n".to_string(),
                }),
            ],
            DocSection::Document,
        ))
    }
}

fn rest_of_line(
    tokens: &TokenList,
    other_doc_locations: &mut CompilerGlobals,
) -> Result<NodeList, String> {
    let len = tokens.len();
    let mut start = 1;
    while start < len {
        if let Token::Whitespace(_) = tokens[start] {
            start += 1;
        } else {
            break;
        }
    }
    node_list(tokens.clone(), start, len - 1, other_doc_locations)
}
