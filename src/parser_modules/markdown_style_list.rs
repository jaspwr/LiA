use std::rc::Rc;

use crate::bracket_depth::BrackDepths;
use crate::hierachy_construction::{
    node_list, CompilerGlobals, IndentationType, NodeParser, ParseResult,
};
use crate::hierarchy::{DocSection, Node, TexEnvironment, Text};
use crate::token::*;
use crate::tokeniser::TokenList;
use crate::utils::{count_indentation, format_error_string, delta_bracket_depth};

#[derive(Default)]
pub struct LiaMardownListParser {
    initial_indentation_depth: usize,
    curly_depth: i32,
}

#[allow(unused)]
impl NodeParser for LiaMardownListParser {
    fn is_opener(
        &mut self,
        token: &Token,
        identation: i32,
        other_doc_locations: &mut CompilerGlobals,
    ) -> bool {
        self.curly_depth = -1;
        match token {
            Token::LiaMarkDown(text, _) => {
                if text == "*" {
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
        token: &Token,
        next_token: &Token,
        next_token_no_white_space: &Token,
        bracket_depths: &BrackDepths,
    ) -> bool {
        if self.curly_depth == -1 {
            self.curly_depth = bracket_depths.curly;
        }
        bracket_depths.curly == self.curly_depth
            && match token {
                Token::Newline => match next_token_no_white_space {
                    Token::LiaMarkDown(text, _) => text != "*",
                    _ => true,
                },
                _ => false,
            }
    }

    fn parse(
        &mut self,
        tokens: TokenList,
        indentation_type: Option<IndentationType>,
        other_doc_locations: &mut CompilerGlobals,
    ) -> ParseResult {
        let mut indentation: usize = self.initial_indentation_depth;
        let mut indentation_type = indentation_type;
        let mut pre_indentation = self.initial_indentation_depth;
        let mut item_count = 0;
        let mut inner_nodes: TokenList = vec![Token::Newline];
        let mut brack_depth = BrackDepths::default();
        for i in 0..tokens.len() {
            brack_depth += delta_bracket_depth(&tokens[i]);
            if item_count > 0 {
                count_indentation(&tokens, i, &mut indentation, &mut indentation_type);
            }
            match &tokens[i] {
                Token::LiaMarkDown(md, loc) => {
                    if md == "*" && brack_depth.curly == 0 {
                        if let Some(value) = list_item(
                            &mut item_count,
                            indentation,
                            &mut pre_indentation,
                            loc,
                            &mut inner_nodes,
                            i,
                            &tokens,
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
                    name: "itemize".to_string(),
                    args: vec![],
                    children: node_list(
                        inner_nodes.clone(),
                        0,
                        inner_nodes.len(),
                        other_doc_locations,
                    )?,
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
    tokens: &Vec<Token>,
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
    inner_nodes.push(Token::Misc("itemize".to_string(), Location::default()));
    inner_nodes.push(Token::Misc("}".to_string(), Location::default()));
    inner_nodes.push(Token::Newline);
}

fn append_closer(inner_nodes: &mut Vec<Token>) {
    inner_nodes.push(Token::TexCommand("\\end".to_string(), Location::default()));
    inner_nodes.push(Token::Misc("{".to_string(), Location::default()));
    inner_nodes.push(Token::Misc("itemize".to_string(), Location::default()));
    inner_nodes.push(Token::Misc("}".to_string(), Location::default()));
    inner_nodes.push(Token::Newline);
}
