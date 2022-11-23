use std::rc::Rc;

use crate::parser_modules::environments::LiaEnvParser;
use crate::parser_modules::imports::LiaUseParser;
use crate::parser_modules::markdown_style_list::LiaMardownListParser;
use crate::parser_modules::markdown_style_section::LiaMarkDownSections;
use crate::parser_modules::tex_command::TexCommandParser;
use crate::parser_modules::variables::LiaVariableParser;
use crate::tokeniser::*;
use crate::hierarchy::*;
use crate::utils::count_whitespace;
use crate::utils::delta_bracket_depth;

pub fn contruct_doc(tokens: TokenList) -> Doc {
    let len = tokens.len();
    let doc = Doc { children: node_list(tokens, 0, len) };
    doc
}

pub fn node_list (tokens: TokenList, start: usize, end: usize) -> NodeList {
    // TODO: split into multiple functions
    let node_parsers: [Rc<dyn NodeParser>; 6] = [
        Rc::new(LiaMarkDownSections::default()),
        Rc::new(TexCommandParser::default()),
        Rc::new(LiaEnvParser::default()),
        Rc::new(LiaUseParser::default()),
        Rc::new(LiaVariableParser::default()),
        Rc::new(LiaMardownListParser::default()),
    ];

    let mut items: NodeList = Vec::new();
    let mut child_tokens_buffer: TokenList = Vec::new();
    let mut in_parser_module: Option<usize> = None;
    let mut bracket_depths = BrackDepths::default();
    if start > tokens.len() || end > tokens.len() {
        panic!("start or end is out of bounds");
    }
    for i in start..end {
        println!("AAA: {:?}", tokens[i]);

        let mut pushed_token_flag = false;
        bracket_depths += delta_bracket_depth(&tokens[i]);
        
        if let Some(m) = in_parser_module {
            let next_token = &tokens[if i + 1 < tokens.len() { i + 1 } else { i }];
            let whitespace = count_whitespace(&tokens, i);
            let next_token_no_whitespace = &tokens[if i + whitespace < tokens.len() { i + whitespace } else { i }];
            if node_parsers[m].is_closer(&tokens[i], next_token, next_token_no_whitespace, &bracket_depths) {
                println!("END {:?}", tokens[i]);
                child_tokens_buffer.push(tokens[i].clone());
                pushed_token_flag = true;
                items.extend(node_parsers[m].parse(child_tokens_buffer.clone()));
                child_tokens_buffer.clear();
                in_parser_module = None;
                for module in &node_parsers {
                    if (*module).is_target(&tokens[i]) {
                        println!("START {:?}", tokens[i]);
                        in_parser_module = Some(i);
                    }
                }
            }
        } else {
            for j in 0..node_parsers.len() {
                if (node_parsers[j]).is_target(&tokens[i]) {
                    items.push(text_node(&child_tokens_buffer));
                    child_tokens_buffer.clear();
                    println!("START {:?}", tokens[i]);
                    in_parser_module = Some(j);
                    if let Some(m) = in_parser_module {
                        let next_token = &tokens[if i + 1 < tokens.len() { i + 1 } else { i }];
                        let whitespace = count_whitespace(&tokens, i);
                        let next_token_no_whitespace = &tokens[if i + whitespace < tokens.len() { i + whitespace } else { i }];
                        if node_parsers[m].is_closer(&tokens[i], next_token, next_token_no_whitespace, &bracket_depths) {
                            child_tokens_buffer.push(tokens[i].clone());
                            pushed_token_flag = true;
                            println!("END {:?}", tokens[i]);
                            items.extend(node_parsers[m].parse(child_tokens_buffer.clone()));
                            child_tokens_buffer.clear();
                            in_parser_module = None;
                        }
                    }
                }
            }
        }
        if !pushed_token_flag {
            child_tokens_buffer.push(tokens[i].clone());
        }
    }
    items.push(text_node(&child_tokens_buffer));
    items
}

fn text_node (tokens: &TokenList) -> Rc<dyn Node> {
    let mut text = String::new();
    for token in tokens {
        match token {
            Token::Nothing(text_) => { text.push_str(&text_); },
            Token::Whitespace(space) => { if space.contains(" ") { text.push_str(&" ".to_string()); }},
            Token::Newline => { text.push_str(&"\n".to_string()); },
            _ => { }
        }
    }
    Rc::new( Text { text })
}



use std::ops::Add;
use std::ops::AddAssign;

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

pub trait NodeParser {
    fn is_target(&self, token: &Token) -> bool;
    fn is_closer(&self, token: &Token, next_token: &Token, next_token_no_white_space: &Token, bracket_depths: &BrackDepths) -> bool;
    fn parse (&self, tokens: TokenList) -> Vec<Rc<dyn Node>>;
}