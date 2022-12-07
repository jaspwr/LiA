use std::rc::Rc;

use crate::token::*;
use crate::hierarchy::*;
use crate::bracket_depth::BrackDepths;
use crate::utils::{delta_bracket_depth, parse_args};
use crate::tokeniser::TokenList;
use crate::hierachy_construction::*;

#[derive(Default)]
pub struct LiaUseParser {}

#[allow(unused)]
impl NodeParser for LiaUseParser {
    fn is_opener(&mut self, token: &Token, identation: i32, other_doc_locations: &mut CompilerGlobals) -> bool {
        match token {
            Token::LiaKeyword(k, _) => { k == "use" },
            _ => { false }
        }
    }

    fn is_closer(&mut self, token: &Token, next_token: &Token, next_token_no_white_space: &Token, bracket_depths: &BrackDepths) -> bool {
        match token {
            Token::Newline => { bracket_depths.curly == 0 },
            _ => { false }
        }
    }

    fn parse (&mut self, tokens: TokenList, indentation_type: Option<IndentationType>, other_doc_locations: &mut CompilerGlobals) -> ParseResult {
        let mut imports: Vec<ArgList> = Vec::new();

        let len = tokens.len();
        
        imports.push(parse_to_args(tokens.clone(), 1, other_doc_locations)?);
        let mut start = 1;
        while start < len {
            match &tokens[start] {
                Token::Nothing(sym, _) => {
                    if sym == "," {
                        imports.push(parse_to_args(tokens.clone(), start+1, other_doc_locations)?);
                    }
                },
                _ => {}
            };
            start += 1; 
        }

        let mut ret: NodeList = Vec::new();
        imports.into_iter().for_each(|args| {
            ret.push(Rc::new( TexCommand {
                command: "usepackage".to_string(),
                args
            }) as Rc<dyn Node>);
            ret.push(Rc::new(Text { text: "\n".to_string() }));
        });
        Ok((ret, DocSection::Imports))
    }
}

fn parse_to_args (tokens: TokenList, start: usize, other_doc_locations: &mut CompilerGlobals) -> Result<ArgList, String> {
    let len = tokens.len();
    let mut start = start;
    while start < len {
        if let Token::Whitespace(_) = tokens[start] { start += 1; } else { break; }
    }
    let mut bracket_depth: BrackDepths = BrackDepths { curly: 0, square: 0, round: 0 };
    let mut end = start;
    while end < len {
        bracket_depth += delta_bracket_depth(&tokens[end]);
        if bracket_depth.curly == 0 && bracket_depth.square == 0 {
            if end + 1 >= len { break; }
            if let Token::Nothing(s, _) = &tokens[end+1] {
                if s == "[" || s == "{" {
                    end += 1;
                    continue;
                } else {
                    end += 1;
                    break;
                }
            } else {
                end += 1;
                break;
            }
        } else {
            end += 1;
        }
    }
    let mut args: ArgList = parse_args(&tokens, start, end, other_doc_locations)?;
    if end + 1 > len {
        panic!("No package name");
    }
    if args.len() == 0 {
        end -= 1;
    }
    args.push(Arg { arg: node_list(tokens, end, end+1, other_doc_locations)?, arg_type: ArgType::Curly });
    Ok(args)
}