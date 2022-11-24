use std::rc::Rc;

use crate::{tokeniser::{Token, TokenList}, hierachy_construction::{BrackDepths, NodeParser, node_list, IndentationType}, hierarchy::{Node, TexCommand, Arg, ArgType, TexEnvironment, NodeList}, utils::{parse_args, count_indentation}};

#[derive(Default)]
pub struct LiaMardownListParser {
    initial_indentation_depth: usize
}

impl NodeParser for LiaMardownListParser {
    fn is_target(&mut self, token: &Token, identation: i32) -> bool {
        match token {
            Token::LiaMarkDown(text) => { if text == "*" {
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
                    Token::LiaMarkDown(text) => { text != "*" },
                    _ => { true }
                }
            },
            _ => { false }
        }
    }

    fn parse (&mut self, tokens: TokenList, indentation_type: Option<IndentationType>) -> Vec<Rc<dyn Node>> {
        // let command = match &tokens[0] {
        //     Token::LiaVariable(command) => { &command[1..] },
        //     _ => { todo!() }
        // }.to_string();
        let mut indentation: usize = self.initial_indentation_depth;
        let mut indentation_type = indentation_type;
        let mut pre_indentation = self.initial_indentation_depth;
        let mut item_count = 0;
        let mut inner_nodes: TokenList = vec![];
        for i in 0..tokens.len() {
            if item_count > 0 {
                count_indentation(&tokens, i, &mut indentation, &mut indentation_type);
            }
            match &tokens[i] {
                Token::LiaMarkDown(md) => { 
                    if md == "*" {
                        item_count += 1;
                        if indentation > pre_indentation {
                            inner_nodes.push(Token::TexCommand("\\begin".to_string()));
                            inner_nodes.push(Token::Nothing("{".to_string()));
                            inner_nodes.push(Token::Nothing("itemize".to_string()));
                            inner_nodes.push(Token::Nothing("}".to_string()));
                            inner_nodes.push(Token::Newline);
                        } else if indentation < pre_indentation {
                            inner_nodes.push(Token::TexCommand("\\end".to_string()));
                            inner_nodes.push(Token::Nothing("{".to_string()));
                            inner_nodes.push(Token::Nothing("itemize".to_string()));
                            inner_nodes.push(Token::Nothing("}".to_string()));
                            inner_nodes.push(Token::Newline);
                        }
                        inner_nodes.push(Token::TexCommand("\\item".to_string()));
                        inner_nodes.push(Token::Whitespace(" ".to_string()));
                        pre_indentation = indentation;
                    } else {
                        inner_nodes.push(tokens[i].clone());
                    }
                },
                _ => { inner_nodes.push(tokens[i].clone()); }
            }
        }
        while pre_indentation > self.initial_indentation_depth {
            inner_nodes.push(Token::TexCommand("\\end".to_string()));
            inner_nodes.push(Token::Nothing("{".to_string()));
            inner_nodes.push(Token::Nothing("itemize".to_string()));
            inner_nodes.push(Token::Nothing("}".to_string()));
            inner_nodes.push(Token::Newline);
            pre_indentation -= 1;
        }
        vec!{Rc::new( TexEnvironment {
            name: "itemize".to_string(),
            args: vec![],
            children: node_list(inner_nodes.clone(), 0, inner_nodes.len())
        })}
    }
}