use std::rc::Rc;

use crate::bracket_depth::BrackDepths;
use crate::document::*;
use crate::parse::*;
use crate::token::*;
use crate::tokenize::TokenList;
use crate::utils::{delta_bracket_depth, parse_args};

#[derive(Default)]
pub struct LiaUseParser {
    curly_depth: i32,
}

impl NodeParser for LiaUseParser {
    fn is_opener(
        &mut self,
        tokens: &[Token],
        cursor: usize,
        _identation: i32,
        _other_doc_locations: &mut CompilerGlobals,
    ) -> bool {
        self.curly_depth = -1;
        match &tokens[cursor] {
            Token::LiaKeyword(k, _) => k == "use",
            _ => false,
        }
    }

    // fn is_closer(
    //     &mut self,
    //     token: &Token,
    //     next_token: &Token,
    //     next_token_no_white_space: &Token,
    //     bracket_depths: &BrackDepths,
    // ) -> bool {
    //     if self.curly_depth == -1 {
    //         self.curly_depth = bracket_depths.curly;
    //     }
    //     match token {
    //         Token::Newline => bracket_depths.curly == self.curly_depth,
    //         _ => false,
    //     }
    // }

    fn parse(
        &mut self,
        tokens: &[Token],
        range_start: usize,
        range_end: usize,
        indentation_type: Option<IndentationType>,
        other_doc_locations: &mut CompilerGlobals,
    ) -> ParseResult {
        let tokens = &tokens[range_start..=range_end];

        let mut imports: Vec<ArgList> = Vec::new();

        let len = tokens.len();

        imports.push(parse_to_args(
            tokens,
            1,
            self.curly_depth,
            other_doc_locations,
        )?);
        let mut start = 1;
        while start < len {
            if let Token::Misc(sym, _) = &tokens[start] {
                if sym == "," {
                    imports.push(parse_to_args(
                        tokens,
                        start + 1,
                        self.curly_depth,
                        other_doc_locations,
                    )?);
                }
            };
            start += 1;
        }

        let mut ret: NodeList = Vec::new();
        imports.into_iter().for_each(|args| {
            ret.push(Rc::new(TexCommand {
                command: "usepackage".to_string(),
                args,
            }) as Rc<dyn Node>);
            ret.push(Rc::new(Text {
                text: "\n".to_string(),
            }));
        });
        Ok((ret, DocSection::Imports))
    }

    fn is_closer(&mut self, tokens: &[Token], cursor: usize, bracket_depths: &BrackDepths) -> bool {
        let token = &tokens[cursor];
            
        if self.curly_depth == -1 {
            self.curly_depth = bracket_depths.curly;
        }
        match token {
            Token::Newline => bracket_depths.curly == self.curly_depth,
            _ => false,
        }
    }
}

fn parse_to_args(
    tokens: TokenList,
    start: usize,
    curly_depth: i32,
    other_doc_locations: &mut CompilerGlobals,
) -> Result<ArgList, String> {
    let len = tokens.len();
    let mut start = start;
    while start < len {
        if let Token::Whitespace(_) = tokens[start] {
            start += 1;
        } else {
            break;
        }
    }
    let mut bracket_depth: BrackDepths = BrackDepths {
        curly: 0,
        square: 0,
        round: 0,
    };
    let mut end = start;
    while end < len {
        bracket_depth += delta_bracket_depth(&tokens[end]);
        if bracket_depth.curly == curly_depth && bracket_depth.square == 0 {
            if end + 1 >= len {
                break;
            }
            if let Token::Misc(s, _) = &tokens[end + 1] {
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
    if args.is_empty() {
        end -= 1;
    }
    args.push(Arg {
        arg: node_list(tokens, end, end + 1, other_doc_locations)?,
        arg_type: ArgType::Curly,
    });
    Ok(args)
}
