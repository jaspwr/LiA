use std::rc::Rc;

use crate::token::*;
use crate::bracket_depth::BrackDepths;
use crate::utils::{count_indentation, format_error_string};
use crate::hierarchy::{TexEnvironment, Node, DocSection, Text};
use crate::hierachy_construction::{NodeParser, node_list, IndentationType, ParseResult, CompilerGlobals};
use crate::tokeniser::TokenList;

#[derive(Default)]
pub struct LiaMardownEnumListParser {
    initial_indentation_depth: usize,
    not_start_of_line: bool
}

#[allow(unused)]
impl NodeParser for LiaMardownEnumListParser {
    fn is_opener(&mut self, token: &Token, identation: i32, other_doc_locations: &mut CompilerGlobals) -> bool {
        if !other_doc_locations.feature_status_list.enumerated_lists.is_supported() { return false; }
        if let Token::Newline = token { 
            self.not_start_of_line = false; 
            return false; 
        } else {
            if (!self.not_start_of_line) {
                if let Token::Whitespace(_) = token {} else {
                    self.not_start_of_line = true;
                }
            } else {
                return false;
            }
        }
        match token {
            Token::Nothing(text, _) => { if is_list_number(text.to_string()) {
                self.initial_indentation_depth = identation as usize;
                true
            } else {
                false
            } },
            _ => { false }
        }
    }

    fn is_closer(&mut self, token: &Token, next_token: &Token, next_token_no_white_space: &Token, bracket_depths: &BrackDepths) -> bool {
        bracket_depths.curly == 0
        && match token {
            Token::Newline => { 
                match next_token_no_white_space {
                    Token::Nothing(text, _) => { !is_list_number(next_token_no_white_space.stringify()) },
                    _ => { true }
                }
            },
            _ => { false }
        }
    }

    fn parse (&mut self, tokens: TokenList, indentation_type: Option<IndentationType>, other_doc_locations: &mut CompilerGlobals) -> ParseResult {
        let mut indentation: usize = self.initial_indentation_depth;
        let mut indentation_type = indentation_type;
        let mut pre_indentation = self.initial_indentation_depth;
        let mut item_count = 0;
        let mut inner_nodes: TokenList = vec![Token::Newline];
        for i in 0..tokens.len() {
            if item_count > 0 {
                count_indentation(&tokens, i, &mut indentation, &mut indentation_type);
            }
            match &tokens[i] {
                Token::Nothing(t, loc) => { 
                    if is_list_number(t.to_string()) {
                        if let Some(value) = list_item(&mut item_count, indentation, 
                            &mut pre_indentation, loc, &mut inner_nodes, i, &tokens) {
                            return value;
                        }
                    } else {
                        inner_nodes.push(tokens[i].clone());
                    }
                },
                _ => { inner_nodes.push(tokens[i].clone()); }
            }
        }
        while pre_indentation > self.initial_indentation_depth {
            append_closer(&mut inner_nodes);
            pre_indentation -= 1;
        }
        Ok((vec!{Rc::new( TexEnvironment {
            name: "enumerate".to_string(),
            args: vec![],
            children: node_list(inner_nodes.clone(), 0, inner_nodes.len(), other_doc_locations)?
        }), Rc::new( Text {text: "\n".to_string() } )}, DocSection::Document))
    }
}

fn list_item(item_count: &mut i32, indentation: usize, pre_indentation: &mut usize, loc: &Location, 
    inner_nodes: &mut Vec<Token>, i: usize, tokens: &Vec<Token>) -> Option<Result<(Vec<Rc<dyn Node>>, DocSection), String>> {
    *item_count += 1;
    if indentation > *pre_indentation {
        if indentation - *pre_indentation > 1 {
            return Some(format_error_string("Indentation error. Nested item list was indented too far.".to_string(), *loc));
        }
        append_opener(inner_nodes);
    } else if indentation < *pre_indentation {
        let diff = *pre_indentation - indentation;
        for _ in 0..diff {
            append_closer(inner_nodes);
        }
    }
    inner_nodes.push(Token::TexCommand("\\item".to_string(), Location::default()));
    if i + 1 < tokens.len() {
        if let Token::Whitespace(_) = &tokens[i + 1] {
            {}
        } else {
            inner_nodes.push(Token::Whitespace(" ".to_string()));
        }
    }
    *pre_indentation = indentation;
    None
}

fn append_opener(inner_nodes: &mut Vec<Token>) {
    inner_nodes.push(Token::TexCommand("\\begin".to_string(), Location::default()));
    inner_nodes.push(Token::Nothing("{".to_string(), Location::default()));
    inner_nodes.push(Token::Nothing("enumerate".to_string(), Location::default()));
    inner_nodes.push(Token::Nothing("}".to_string(), Location::default()));
    inner_nodes.push(Token::Newline);
}

fn append_closer(inner_nodes: &mut Vec<Token>) {
    inner_nodes.push(Token::TexCommand("\\end".to_string(), Location::default()));
    inner_nodes.push(Token::Nothing("{".to_string(), Location::default()));
    inner_nodes.push(Token::Nothing("enumerate".to_string(), Location::default()));
    inner_nodes.push(Token::Nothing("}".to_string(), Location::default()));
    inner_nodes.push(Token::Newline);
}

fn is_list_number(text: String) -> bool {
    if !text.ends_with('.') { return false; }
    if !text.starts_with(|c: char| c.is_numeric()) { return false; }
    text.chars().all(|c| c.is_numeric() || c == '.')
}