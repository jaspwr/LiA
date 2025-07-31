use std::rc::Rc;

use crate::bracket_depth::BrackDepths;
use crate::compiler::Job;
use crate::document::*;
use crate::feature_matrix::get_status_list;
use crate::feature_matrix::FeatureStatusList;
use crate::parser_modules::bold_italic::BoldItalic;
use crate::parser_modules::comments::Comment;
use crate::parser_modules::enumerated_list::LiaMardownEnumListParser;
use crate::parser_modules::environments::LiaEnvParser;
use crate::parser_modules::equation::LiaEquation;
use crate::parser_modules::imports::LiaUseParser;
use crate::parser_modules::inline_code::InlineCode;
use crate::parser_modules::list::LiaMardownListParser;
use crate::parser_modules::section::LiaMarkDownSections;
use crate::parser_modules::tex_command::TexCommandParser;
use crate::parser_modules::variables::Function;
use crate::parser_modules::variables::LiaVariableParser;
use crate::token::*;
use crate::tokenize::*;
use crate::utils::{count_indentation, delta_bracket_depth};

#[derive(Default)]
pub struct CompilerGlobals {
    imps: NodeList,
    decs: NodeList,
    pub fucntions: Vec<Function>,
    pub feature_status_list: FeatureStatusList,
    pub job: Job,
}

pub fn parse(tokens: TokenList, job: Job) -> Result<Doc, String> {
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

    let mut node_parsers: [Box<dyn NodeParser>; 11] = [
        Box::new(LiaMarkDownSections::default()),
        Box::new(TexCommandParser::default()),
        Box::new(LiaEnvParser::default()),
        Box::new(LiaUseParser::default()),
        Box::new(LiaVariableParser::default()),
        Box::new(LiaMardownListParser::default()),
        Box::new(BoldItalic::default()),
        Box::new(LiaEquation::default()),
        Box::new(LiaMardownEnumListParser::default()),
        Box::new(InlineCode::default()),
        Box::new(Comment::default()),
    ];

    let mut items: NodeList = Vec::new();
    let mut in_parser_module: Option<usize> = None;
    let mut bracket_depths = BrackDepths::default();
    let mut indentation = 0;
    let mut range_started = 0;
    let mut indentation_type: Option<IndentationType> = None;
    let mut bracket_depths_at_start_of_module = BrackDepths::default();

    let mut i = start;

    while i < end {
        if let Token::Whitespace(_) = tokens[i] {
            i += 1;
        } else {
            break;
        }
    }

    'outer: while i < end {
        bracket_depths += delta_bracket_depth(&tokens[i]);

        count_indentation(&tokens, i, &mut indentation, &mut indentation_type);

        if let Some(m) = in_parser_module {
            if node_parsers[m].is_closer(
                tokens,
                i,
                &bracket_depths,
                &bracket_depths_at_start_of_module,
            ) {
                let (nodes, section) = node_parsers[m].parse(
                    tokens,
                    range_started,
                    i,
                    indentation_type,
                    other_doc_locations,
                )?;

                match section {
                    DocSection::Imports => other_doc_locations.imps.extend(nodes),
                    DocSection::Declarations => other_doc_locations.decs.extend(nodes),
                    DocSection::Document => items.extend(nodes),
                }

                in_parser_module = None;
            }
        } else {
            for j in 0..node_parsers.len() {
                if (node_parsers[j]).is_opener(tokens, i, indentation as i32, other_doc_locations) {
                    in_parser_module = Some(j);
                    range_started = i;
                    bracket_depths_at_start_of_module = bracket_depths;
                    continue 'outer;
                }
            }

            items.push(text_node(&[tokens[i].clone()])?);
        }

        i += 1;
    }

    Ok(items)
}

fn text_node(tokens: &[Token]) -> Result<Rc<dyn Node>, String> {
    let mut text = String::new();
    for token in tokens {
        match token {
            Token::Misc(text_, _) => {
                text.push_str(text_);
            }
            Token::Whitespace(space) => {
                if space.contains(" ") {
                    text.push(' ');
                }
            }
            Token::Newline => {
                text.push('\n');
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
                return Err("Environment was opened but never closed.".to_string())
            }
        }
    }
    Ok(Rc::new(Text { text }))
}

pub type ParseResult = Result<(Vec<Rc<dyn Node>>, DocSection), String>;
pub trait NodeParser {
    fn is_opener(
        &mut self,
        tokens: &[Token],
        cursor: usize,
        identation: i32,
        other_doc_locations: &mut CompilerGlobals,
    ) -> bool;
    fn is_closer(
        &mut self,
        tokens: &[Token],
        cursor: usize,
        bracket_depths: &BrackDepths,
        start_bracket_depths: &BrackDepths,
    ) -> bool;
    fn parse(
        &mut self,
        tokens: &[Token],
        range_start: usize,
        range_end: usize,
        indentation_type: Option<IndentationType>,
        other_doc_locations: &mut CompilerGlobals,
    ) -> ParseResult;
}

#[derive(Clone, Copy)]
pub enum IndentationType {
    Space(u8),
    Tab,
}
