use std::rc::Rc;

use crate::tokeniser::TokenList;
use crate::bracket_depth::BrackDepths;
use crate::hierarchy::*;
use crate::token::*;
use crate::hierachy_construction::*;
use crate::utils::format_error_string;

#[derive(Default)]
pub struct LiaMarkDownSections {}

#[allow(unused)]
impl NodeParser for LiaMarkDownSections {
    fn is_opener(&mut self, token: &Token, identation: i32) -> bool {
        match token {
            Token::LiaMarkDown(text, _) => { 
                text.starts_with("#")
            },
            _ => { false }
        }
    }

    fn is_closer(&mut self, token: &Token, next_token: &Token, next_token_no_white_space: &Token, bracket_depths: &BrackDepths) -> bool {
        match token {
            Token::Newline => { bracket_depths.curly == 0 },
            _ => { false }
        }
    }

    fn parse (&mut self, tokens: TokenList, indentation_type: Option<IndentationType>, 
        other_doc_locations: &mut OtherDocLocations) -> ParseResult {
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
                        format!{"Lines opened with '#' will automatically be assumed to be a header. \"{}\" is not a valid header command. If you don't want this to parse as a header, add a '\\' to escape it.", 
                        hash}, 
                        *loc) }
                }
            },
            _ => { panic!("Should not be here.") }
        }.to_string();
        Ok((vec![Rc::new(TexCommand {
            command,
            args: vec![Arg {
                    arg: {
                        rest_of_line(&tokens, other_doc_locations)?
                    },
                    arg_type: ArgType::Curly
                }]
        }), Rc::new(Text { text: "\n".to_string() })
        ], DocSection::Document))
    }
}

fn rest_of_line(tokens: &TokenList, other_doc_locations: &mut OtherDocLocations) -> Result<NodeList, String> {
    let len = tokens.len();
    let mut start = 1;
    while start < len {
        if let Token::Whitespace(_) = tokens[start] { start += 1; } else { break; }
    }
    Ok(node_list(tokens.clone(), start, len-1, other_doc_locations)?)
}