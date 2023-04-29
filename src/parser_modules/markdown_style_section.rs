use std::rc::Rc;

use crate::bracket_depth::BrackDepths;
use crate::hierachy_construction::*;
use crate::hierarchy::*;
use crate::token::*;
use crate::tokeniser::TokenList;
use crate::utils::format_error_string;

#[derive(Default)]
pub struct LiaMarkDownSections {
    curly_depth: i32,
}

#[allow(unused)]
impl NodeParser for LiaMarkDownSections {
    fn is_opener(
        &mut self,
        token: &Token,
        identation: i32,
        other_doc_locations: &mut CompilerGlobals,
    ) -> bool {
        self.curly_depth = -1;
        match token {
            Token::LiaMarkDown(text, _) => text.starts_with("#"),
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
            Token::Newline => bracket_depths.curly == self.curly_depth,
            _ => false,
        }
    }

    fn parse(
        &mut self,
        tokens: TokenList,
        indentation_type: Option<IndentationType>,
        other_doc_locations: &mut CompilerGlobals,
    ) -> ParseResult {
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
                        format!{"Lines opened with '#' will automatically be assumed to be a section. \"{}\" is not a valid section command. If you don't want this to parse as a section, add a '\\' to escape it.",
                        hash},
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
    Ok(node_list(
        tokens.clone(),
        start,
        len - 1,
        other_doc_locations,
    )?)
}
