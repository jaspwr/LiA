use std::path::PathBuf;
use std::rc::Rc;

use crate::bracket_depth::BrackDepths;
use crate::document::*;
use crate::parse::*;
use crate::token::*;
use crate::tokenize::TokenList;
use crate::utils::untokenise;
use crate::utils::{delta_bracket_depth, parse_args};

#[derive(Default)]
pub struct LiaUseParser {}

impl NodeParser for LiaUseParser {
    fn is_opener(
        &mut self,
        tokens: &[Token],
        cursor: usize,
        _identation: i32,
        _other_doc_locations: &mut CompilerGlobals,
    ) -> bool {
        match &tokens[cursor] {
            Token::LiaKeyword(k, _) => k == "use",
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
        // TODO: clean up. i was very bad at rust when i wrote this.

        let tokens = &tokens[range_start..=range_end];

        let raw = untokenise(&tokens[1..]);
        let raw = raw.trim();
        if raw.ends_with(".lia") {
            let mut path = PathBuf::from(&other_doc_locations.job.input_path)
                .parent()
                .unwrap()
                .to_path_buf();
            path.push(raw);

            if !path.exists() {
                return Err(format!(
                    "{} The path `{}` could not be found",
                    tokens.first().unwrap().get_location().stringify(),
                    path.display()
                ));
            }

            let tokens = crate::tokenize::to_tokens(
                std::fs::read_to_string(&path).map_err(|e| e.to_string())?,
            );

            // HACK
            let tmp = other_doc_locations.job.input_path.clone();
            other_doc_locations.job.input_path = path.to_string_lossy().to_string();

            let nodes = node_list(&tokens, 0, tokens.len(), other_doc_locations)?;

            other_doc_locations.job.input_path = tmp;

            return Ok((nodes, DocSection::Document));
        }

        let mut imports: Vec<ArgList> = vec![];

        let len = tokens.len();

        let pkgs = parse_to_args(tokens, 1, other_doc_locations)?;
        imports.push(pkgs);

        let mut start = 1;
        while start < len {
            if let Token::Misc(sym, _) = &tokens[start] {
                if sym == "," {
                    let pkgs = parse_to_args(tokens, start + 1, other_doc_locations)?;
                    imports.push(pkgs);
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

    fn is_closer(
        &mut self,
        tokens: &[Token],
        cursor: usize,
        bracket_depths: &BrackDepths,
        start_bracket_depths: &BrackDepths,
    ) -> bool {
        let token = &tokens[cursor];

        match token {
            Token::Newline => bracket_depths.curly == start_bracket_depths.curly,
            _ => false,
        }
    }
}

fn parse_to_args(
    tokens: TokenList,
    start: usize,
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
        if bracket_depth.curly == 0 && bracket_depth.square == 0 {
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
