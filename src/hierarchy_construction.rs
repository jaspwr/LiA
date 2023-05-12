use std::rc::Rc;

use crate::bracket_depth::BrackDepths;
use crate::compiler::Job;
use crate::feature_matrix::get_status_list;
use crate::feature_matrix::FeatureStatusList;
use crate::hierarchy::*;
use crate::parser_modules::bold_italic::BoldItalic;
use crate::parser_modules::comments::Comment;
use crate::parser_modules::enumerated_list::LiaMardownEnumListParser;
use crate::parser_modules::environments::LiaEnvParser;
use crate::parser_modules::equation::LiaEquation;
use crate::parser_modules::imports::LiaUseParser;
use crate::parser_modules::list::LiaMardownListParser;
use crate::parser_modules::section::LiaMarkDownSections;
use crate::parser_modules::tex_command::TexCommandParser;
use crate::parser_modules::variables::Function;
use crate::parser_modules::variables::LiaVariableParser;
use crate::token::*;
use crate::tokeniser::*;
use crate::utils::{count_indentation, count_whitespace, delta_bracket_depth};

#[derive(Default)]
pub struct CompilerGlobals {
    imps: NodeList,
    decs: NodeList,
    pub fucntions: Vec<Function>,
    pub feature_status_list: FeatureStatusList,
    pub job: Job,
}

pub fn contruct_doc(tokens: TokenList, job: Job) -> Result<Doc, String> {
    let len = tokens.len();
    let mut other_doc_locations = CompilerGlobals::default();
    other_doc_locations.job = job;
    other_doc_locations.feature_status_list = get_status_list(env!("CARGO_PKG_VERSION"))?;

    let doc = node_list(tokens, 0, len, &mut other_doc_locations)?;
    let doc = Doc {
        imports: other_doc_locations.imps,
        declarations: other_doc_locations.decs,
        document: doc,
    };
    Ok(doc)
}

pub fn node_list(
    tokens: TokenList,
    start: usize,
    end: usize,
    other_doc_locations: &mut CompilerGlobals,
) -> Result<NodeList, String> {
    // TODO: Refactor this function to be more readable.
    //       It's impossible to work with at the moment.

    let mut node_parsers: [Box<dyn NodeParser>; 10] = [
        Box::new(LiaMarkDownSections::default()),
        Box::new(TexCommandParser::default()),
        Box::new(LiaEnvParser::default()),
        Box::new(LiaUseParser::default()),
        Box::new(LiaVariableParser::default()),
        Box::new(LiaMardownListParser::default()),
        Box::new(BoldItalic::default()),
        Box::new(LiaEquation::default()),
        Box::new(LiaMardownEnumListParser::default()),
        Box::new(Comment::default()),
    ];

    let mut items: NodeList = Vec::new();
    let mut child_tokens_buffer: TokenList = Vec::new();
    let mut in_parser_module: Option<usize> = None;
    let mut bracket_depths = BrackDepths::default();
    let mut indentation = 0;
    let mut indentation_type: Option<IndentationType> = None;

    let start = move_past_whitespace(&tokens, start);
    check_range(start, &tokens, end);

    let mut skip_next_flag = false;
    for i in start..end {
        if skip_next_flag {
            skip_next_flag = false;
            continue;
        }
        let mut pushed_token_flag = false;
        bracket_depths += delta_bracket_depth(&tokens[i]);

        count_indentation(&tokens, i, &mut indentation, &mut indentation_type);

        if let Some(m) = in_parser_module {
            let whitespace = count_whitespace(&tokens, i);
            if node_parsers[m].is_closer(
                &tokens[i],
                &tokens[clamp_index(i, &tokens)],
                &tokens[if i + whitespace < tokens.len() {
                    i + whitespace
                } else {
                    i
                }],
                &bracket_depths,
            ) {
                append_token(
                    &mut child_tokens_buffer,
                    &tokens,
                    i,
                    &mut pushed_token_flag,
                    &mut node_parsers[m],
                    indentation_type,
                    other_doc_locations,
                    &mut items,
                    &mut in_parser_module,
                )?;

                // For single token commands
                for j in 0..node_parsers.len() {
                    if (node_parsers[j]).is_opener(
                        &tokens[i],
                        indentation as i32,
                        other_doc_locations,
                    ) {
                        append_text_node(
                            &mut items,
                            &mut child_tokens_buffer,
                            &mut in_parser_module,
                            j,
                        )?;
                    }
                }
            }
        } else {
            for j in 0..node_parsers.len() {
                if (node_parsers[j]).is_opener(&tokens[i], indentation as i32, other_doc_locations)
                {
                    append_text_node(
                        &mut items,
                        &mut child_tokens_buffer,
                        &mut in_parser_module,
                        j,
                    )?;

                    // For commands that start at the end of another token
                    if let Some(m) = in_parser_module {
                        let whitespace = count_whitespace(&tokens, i);
                        if node_parsers[m].is_closer(
                            &tokens[i],
                            &tokens[clamp_index(i, &tokens)],
                            &tokens[if i + whitespace < tokens.len() {
                                i + whitespace
                            } else {
                                i
                            }],
                            &bracket_depths,
                        ) {
                            append_token(
                                &mut child_tokens_buffer,
                                &tokens,
                                i,
                                &mut pushed_token_flag,
                                &mut node_parsers[m],
                                indentation_type,
                                other_doc_locations,
                                &mut items,
                                &mut in_parser_module,
                            )?;
                        }
                    }
                }
            }
        }
        // This control flow is scuffed need to refactor.
        if !pushed_token_flag {
            child_tokens_buffer.push(tokens[i].clone());
        }
    }
    items.push(text_node(&child_tokens_buffer)?);
    Ok(items)
}

fn move_past_whitespace(tokens: &Vec<Token>, start: usize) -> usize {
    let mut start = start;
    while let Token::Whitespace(_) = &tokens[start] {
        start += 1;
    }
    start
}

fn check_range(start: usize, tokens: &Vec<Token>, end: usize) {
    if start > tokens.len() || end > tokens.len() {
        panic!("start or end is out of bounds");
    }
}

fn clamp_index(i: usize, tokens: &Vec<Token>) -> usize {
    if i + 1 < tokens.len() {
        i + 1
    } else {
        i
    }
}

fn append_token(
    child_tokens_buffer: &mut Vec<Token>,
    tokens: &Vec<Token>,
    i: usize,
    pushed_token_flag: &mut bool,
    node_parser: &mut Box<dyn NodeParser>,
    indentation_type: Option<IndentationType>,
    other_doc_locations: &mut CompilerGlobals,
    items: &mut Vec<Rc<dyn Node>>,
    in_parser_module: &mut Option<usize>,
) -> Result<(), String> {
    child_tokens_buffer.push(tokens[i].clone());
    *pushed_token_flag = true;
    let node = node_parser.parse(
        child_tokens_buffer.clone(),
        indentation_type,
        other_doc_locations,
    )?;
    match node.1 {
        DocSection::Document => items.extend(node.0),
        DocSection::Declarations => other_doc_locations.decs.extend(node.0),
        DocSection::Imports => other_doc_locations.imps.extend(node.0),
    }
    child_tokens_buffer.clear();
    *in_parser_module = None;
    Ok(())
}

fn append_text_node(
    items: &mut Vec<Rc<dyn Node>>,
    child_tokens_buffer: &mut Vec<Token>,
    in_parser_module: &mut Option<usize>,
    j: usize,
) -> Result<(), String> {
    items.push(text_node(&*child_tokens_buffer)?);
    child_tokens_buffer.clear();
    *in_parser_module = Some(j);
    Ok(())
}

fn text_node(tokens: &TokenList) -> Result<Rc<dyn Node>, String> {
    let mut text = String::new();
    for token in tokens {
        match token {
            Token::Misc(text_, _) => {
                text.push_str(&text_);
            }
            Token::Whitespace(space) => {
                if space.contains(" ") {
                    text.push_str(&" ".to_string());
                }
            }
            Token::Newline => {
                text.push_str(&"\n".to_string());
            }
            Token::LiaKeyword(s, loc) => {
                return Err(format! {"{} Malformed {} statement.", loc.stringify(), s})
            }
            Token::LiaMarkDown(s, loc) => {
                return Err(format! {"{} Malformed {} expression.", loc.stringify(), s})
            }
            Token::LiaVariable(s, loc) => {
                return Err(
                    format! {"{} Malformed variable expression for \"{}\".", loc.stringify(), s},
                )
            }
            Token::TexCommand(_, _) => {
                return Err(format! {"Environment was opened but never closed."})
            }
        }
    }
    Ok(Rc::new(Text { text }))
}

pub type ParseResult = Result<(Vec<Rc<dyn Node>>, DocSection), String>;
pub trait NodeParser {
    fn is_opener(
        &mut self,
        token: &Token,
        identation: i32,
        other_doc_locations: &mut CompilerGlobals,
    ) -> bool;
    fn is_closer(
        &mut self,
        token: &Token,
        next_token: &Token,
        next_token_no_white_space: &Token,
        bracket_depths: &BrackDepths,
    ) -> bool;
    fn parse(
        &mut self,
        tokens: TokenList,
        indentation_type: Option<IndentationType>,
        other_doc_locations: &mut CompilerGlobals,
    ) -> ParseResult;
}

#[derive(Clone, Copy)]
pub enum IndentationType {
    Space(u8),
    Tab,
}
