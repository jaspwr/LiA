use std::rc::Rc;

use crate::bracket_depth::BrackDepths;
use crate::document::{DocSection, Node, TexEnvironment, Text};
use crate::parse::{node_list, CompilerGlobals, IndentationType, NodeParser, ParseResult};
use crate::token::*;
use crate::tokenize::TokenList;
use crate::utils::{count_indentation, delta_bracket_depth, format_error_string};

#[derive(Default)]
pub struct LiaMardownEnumListParser {
    initial_indentation_depth: usize,
    not_start_of_line: bool,
}

impl NodeParser for LiaMardownEnumListParser {
    fn is_opener(
        &mut self,
        tokens: &[Token],
        cursor: usize,
        identation: i32,
        other_doc_locations: &mut CompilerGlobals,
    ) -> bool {
        let token = &tokens[cursor];

        if !other_doc_locations
            .feature_status_list
            .enumerated_lists
            .is_supported()
        {
            return false;
        }
        if let Token::Newline = token {
            self.not_start_of_line = false;
            return false;
        } else if !self.not_start_of_line {
            if let Token::Whitespace(_) = token {
            } else {
                self.not_start_of_line = true;
            }
        } else {
            return false;
        }
        match token {
            Token::Misc(text, _) => {
                if is_list_number(text.to_string()) {
                    self.initial_indentation_depth = identation as usize;
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn is_closer(
        &mut self,
        tokens: &[Token],
        cursor: usize,
        bracket_depths: &BrackDepths,
        start_bracket_depths: &BrackDepths,
    ) -> bool {
        let token = &tokens[cursor];
        let next_token_no_white_space =
            &crate::utils::move_past_whitespace(tokens, cursor + 1).unwrap_or(Token::Newline);

        bracket_depths.curly == start_bracket_depths.curly
            && match token {
                Token::Newline => match next_token_no_white_space {
                    Token::Misc(_, _) => !is_list_number(next_token_no_white_space.stringify()),
                    _ => true,
                },
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

        let mut indentation: usize = self.initial_indentation_depth;
        let mut indentation_type = indentation_type;
        let mut pre_indentation = self.initial_indentation_depth;
        let mut item_count = 0;
        let mut inner_nodes: Vec<Token> = vec![Token::Newline];
        let mut brack_depth = BrackDepths::default();
        for i in 0..tokens.len() {
            brack_depth += delta_bracket_depth(&tokens[i]);
            if item_count > 0 {
                count_indentation(&tokens, i, &mut indentation, &mut indentation_type);
            }
            match &tokens[i] {
                Token::Misc(t, loc) => {
                    if is_list_number(t.to_string()) && brack_depth.curly == 0 {
                        if let Some(value) = list_item(
                            &mut item_count,
                            indentation,
                            &mut pre_indentation,
                            loc,
                            &mut inner_nodes,
                            i,
                            tokens,
                        ) {
                            return value;
                        }
                    } else {
                        inner_nodes.push(tokens[i].clone());
                    }
                }
                _ => {
                    inner_nodes.push(tokens[i].clone());
                }
            }
        }
        while pre_indentation > self.initial_indentation_depth {
            append_closer(&mut inner_nodes);
            pre_indentation -= 1;
        }
        Ok((
            vec![
                Rc::new(TexEnvironment {
                    name: "enumerate".to_string(),
                    args: vec![],
                    children: node_list(&inner_nodes, 0, inner_nodes.len(), other_doc_locations)?,
                }),
                Rc::new(Text {
                    text: "\n".to_string(),
                }),
            ],
            DocSection::Document,
        ))
    }
}

fn list_item(
    item_count: &mut i32,
    indentation: usize,
    pre_indentation: &mut usize,
    loc: &Location,
    inner_nodes: &mut Vec<Token>,
    i: usize,
    tokens: TokenList,
) -> Option<Result<(Vec<Rc<dyn Node>>, DocSection), String>> {
    *item_count += 1;
    if indentation > *pre_indentation {
        if indentation - *pre_indentation > 1 {
            return Some(format_error_string(
                "Indentation error. Nested item list was indented too far.".to_string(),
                *loc,
            ));
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
    inner_nodes.push(Token::TexCommand(
        "\\begin".to_string(),
        Location::default(),
    ));
    inner_nodes.push(Token::Misc("{".to_string(), Location::default()));
    inner_nodes.push(Token::Misc("enumerate".to_string(), Location::default()));
    inner_nodes.push(Token::Misc("}".to_string(), Location::default()));
    inner_nodes.push(Token::Newline);
}

fn append_closer(inner_nodes: &mut Vec<Token>) {
    inner_nodes.push(Token::TexCommand("\\end".to_string(), Location::default()));
    inner_nodes.push(Token::Misc("{".to_string(), Location::default()));
    inner_nodes.push(Token::Misc("enumerate".to_string(), Location::default()));
    inner_nodes.push(Token::Misc("}".to_string(), Location::default()));
    inner_nodes.push(Token::Newline);
}

fn is_list_number(text: String) -> bool {
    if !text.ends_with('.') {
        return false;
    }
    if !text.starts_with(|c: char| c.is_numeric()) {
        return false;
    }
    text.chars().all(|c| c.is_numeric() || c == '.')
}
