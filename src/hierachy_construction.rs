use std::rc::Rc;

use crate::parser_modules::environments::LiaEnvParser;
use crate::parser_modules::imports::LiaUseParser;
use crate::parser_modules::markdown_style_list::LiaMardownListParser;
use crate::parser_modules::markdown_style_section::LiaMarkDownSections;
use crate::parser_modules::tex_command::TexCommandParser;
use crate::parser_modules::variables::LiaVariableParser;
use crate::tokeniser::*;
use crate::hierarchy::*;
use crate::utils::count_indentation;
use crate::utils::count_whitespace;
use crate::utils::delta_bracket_depth;
use std::ops::Add;
use std::ops::AddAssign;

#[derive(Default)]
pub struct OtherDocLocations {
    imps: NodeList,
    decs: NodeList,
}

pub fn contruct_doc(tokens: TokenList) -> Result<Doc, String> {
    let len = tokens.len();
    let mut other_doc_locations = OtherDocLocations::default();

    let doc = node_list(tokens, 0, len, &mut other_doc_locations)?;
    let doc = Doc {
        imports: other_doc_locations.imps,
        declarations: other_doc_locations.decs,
        document: doc
    };
    Ok(doc)
}

pub fn node_list (tokens: TokenList, start: usize, end: usize, other_doc_locations: &mut OtherDocLocations) -> Result<NodeList, String> {
    let mut node_parsers: [Box<dyn NodeParser>; 6] = [
        Box::new(LiaMarkDownSections::default()),
        Box::new(TexCommandParser::default()),
        Box::new(LiaEnvParser::default()),
        Box::new(LiaUseParser::default()),
        Box::new(LiaVariableParser::default()),
        Box::new(LiaMardownListParser::default()),
    ];

    let mut items: NodeList = Vec::new();
    let mut child_tokens_buffer: TokenList = Vec::new();
    let mut in_parser_module: Option<usize> = None;
    let mut bracket_depths = BrackDepths::default();
    let mut indentation = 0;
    let mut indentation_type: Option<IndentationType> = None;
    let mut start = start;

    // Remove leading whitespace.
    while let Token::Whitespace(_) = &tokens[start] {
        start += 1;
    }
    if start > tokens.len() || end > tokens.len() {
        panic!("start or end is out of bounds");
    }
    let mut skip_next_flag = false;
    for i in start..end {
        if skip_next_flag { skip_next_flag = false; continue; }
        let mut pushed_token_flag = false;
        bracket_depths += delta_bracket_depth(&tokens[i]);

        count_indentation(&tokens, i, &mut indentation, &mut indentation_type);
        
        if let Some(m) = in_parser_module {
            let whitespace = count_whitespace(&tokens, i);
            if node_parsers[m].is_closer(&tokens[i], 
                &tokens[clamp_index(i, &tokens)],
                &tokens[if i + whitespace < tokens.len() { i + whitespace } else { i }],
                &bracket_depths) {
                    
                    append_token(&mut child_tokens_buffer, &tokens, i, &mut pushed_token_flag, 
                        &mut node_parsers[m], indentation_type, other_doc_locations, 
                        &mut items, &mut in_parser_module)?;

                // For single token commands
                for j in 0..node_parsers.len() {
                    if (node_parsers[j]).is_target(&tokens[i], indentation as i32) {
                        append_text_node(&mut items, &mut child_tokens_buffer, &mut in_parser_module, j)?;
                    }
                }
            }
        } else {
            for j in 0..node_parsers.len() {
                if (node_parsers[j]).is_target(&tokens[i], indentation as i32) {
                    append_text_node(&mut items, &mut child_tokens_buffer, &mut in_parser_module, j)?;
                    
                    // For commands that start at the end of another token
                    if let Some(m) = in_parser_module {
                        let whitespace = count_whitespace(&tokens, i);
                        if node_parsers[m].is_closer(&tokens[i], 
                            &tokens[clamp_index(i, &tokens)],
                            &tokens[if i + whitespace < tokens.len() { i + whitespace } else { i }],
                            &bracket_depths) {

                            append_token(&mut child_tokens_buffer, &tokens, i, &mut pushed_token_flag, 
                                &mut node_parsers[m], indentation_type, other_doc_locations, 
                                &mut items, &mut in_parser_module)?;
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

fn clamp_index(i: usize, tokens: &Vec<Token>) -> usize {
    if i + 1 < tokens.len() { i + 1 } else { i }
}

fn append_token(child_tokens_buffer: &mut Vec<Token>, tokens: &Vec<Token>, i: usize, pushed_token_flag: 
    &mut bool, node_parser: &mut Box<dyn NodeParser>, indentation_type: Option<IndentationType>, other_doc_locations: 
    &mut OtherDocLocations, items: &mut Vec<Rc<dyn Node>>, in_parser_module: &mut Option<usize>) -> Result<(), String> {
    child_tokens_buffer.push(tokens[i].clone());
    *pushed_token_flag = true;
    let node = node_parser.parse(child_tokens_buffer.clone(), indentation_type, other_doc_locations)?;
    match node.1 {
        DocSection::Document => { items.extend(node.0) },
        DocSection::Declarations => { other_doc_locations.decs.extend(node.0)},
        DocSection::Imports => { other_doc_locations.imps.extend(node.0) },
    }
    child_tokens_buffer.clear();
    *in_parser_module = None;
    Ok(())
}

fn append_text_node(items: &mut Vec<Rc<dyn Node>>, child_tokens_buffer: &mut Vec<Token>, in_parser_module: &mut Option<usize>, j: usize) -> Result<(), String> {
    items.push(text_node(&*child_tokens_buffer)?);
    child_tokens_buffer.clear();
    *in_parser_module = Some(j);
    Ok(())
}

fn text_node (tokens: &TokenList) -> Result<Rc<dyn Node>, String> {
    let mut text = String::new();
    for token in tokens {
        match token {
            Token::Nothing(text_, _) => { text.push_str(&text_); },
            Token::Whitespace(space) => { if space.contains(" ") { text.push_str(&" ".to_string()); }},
            Token::Newline => { text.push_str(&"\n".to_string()); },
            Token::LiaKeyword(s, loc) => { return Err(format!{"{} Malformed {} statement.", loc.stringify(), s})},
            Token::LiaMarkDown(s, loc) => { return Err(format!{"{} Malformed {} expression.", loc.stringify(), s})},
            Token::LiaVariable(s, loc) => { return Err(format!{"{} Malformed variable expression for \"{}\".", loc.stringify(), s})},
            _ => { }
        }
    }
    Ok(Rc::new( Text { text }))
}

#[derive(Default, Copy, Clone, PartialEq)]
pub struct BrackDepths {
    pub curly: i32,
    pub square: i32,
    pub round: i32,
}

impl Add for BrackDepths {
    type Output = BrackDepths;

    fn add(self, other: BrackDepths) -> BrackDepths {
        BrackDepths {
            curly: self.curly + other.curly,
            square: self.square + other.square,
            round: self.round + other.round,
        }
    }
}

impl AddAssign for BrackDepths {
    fn add_assign(&mut self, other: BrackDepths) {
        self.curly += other.curly;
        self.square += other.square;
        self.round += other.round;
    }
}

impl BrackDepths {
    pub fn is_zero(&self) -> bool {
        self.curly == 0 && self.square == 0 && self.round == 0
    }
}

pub type ParseResult = Result<(Vec<Rc<dyn Node>>, DocSection), String>;
pub trait NodeParser {
    fn is_target(&mut self, token: &Token, identation: i32) -> bool;
    fn is_closer(&mut self, token: &Token, next_token: &Token, next_token_no_white_space: &Token, bracket_depths: &BrackDepths) -> bool;
    fn parse (&mut self, tokens: TokenList, indentation_type: Option<IndentationType>, other_doc_locations: &mut OtherDocLocations) -> ParseResult;
}

#[derive(Clone, Copy)]
pub enum IndentationType {
    Space(u8),
    Tab
}

pub enum DocSection {
    Imports,
    Declarations,
    Document
}