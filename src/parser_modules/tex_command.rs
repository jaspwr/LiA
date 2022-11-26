use std::rc::Rc;

use crate::{tokeniser::{Token, TokenList}, hierachy_construction::{BrackDepths, NodeParser, IndentationType, ParseResult, DocSection, node_list, OtherDocLocations}, hierarchy::{TexCommand, TexEnvironment, Node, Text}, utils::parse_args};

#[derive(Default, PartialEq, Debug)]
enum EnvParsingState {
    #[default]
    NotEnv,
    BeginOpeningCurly,
    BeginName,
    BeginClosingCurly,
    EndOpeningCurly,
    EndName,
    EndClosingCurly,
    SquareBracketEquationEnv,
}

#[derive(Default)]
pub struct TexCommandParser {
    env_parsing_state: EnvParsingState,
    env_name: String,
    env_depth: i32,
    is_dec: bool,
    next: bool,
}

#[allow(unused)]
impl NodeParser for TexCommandParser {
    fn is_target(&mut self, token: &Token, identation: i32) -> bool {
        self.env_parsing_state = EnvParsingState::NotEnv;
        self.env_depth = 0;
        self.is_dec = false;
        self.next = false;
        println!("is_target: {:?}", token);
        match token {
            Token::TexCommand(com, _) => { 
                if com == "\\begin" { self.env_parsing_state = EnvParsingState::BeginOpeningCurly; }
                else if com == "\\[" { self.env_parsing_state = EnvParsingState::SquareBracketEquationEnv;}                 
                true },
            _ => { false }
        }
    }

    fn is_closer(&mut self, token: &Token, next_token: &Token, next_token_no_white_space: &Token, bracket_depths: &BrackDepths) -> bool {
        if self.is_dec {
            if let Token::Newline = token { return true; } else { return false; }
        }
        if self.next {
            return true;
        }

        match self.env_parsing_state {
            EnvParsingState::NotEnv => {
                bracket_depths.curly == 0 && bracket_depths.square == 0
                && match next_token {
                    Token::Nothing(t, _) => { t != "{" && t != "[" },
                    Token::Newline => { false }, // Consume trailing newline
                    _ => { true }
                } && match next_token_no_white_space {
                    Token::Nothing(t, _) => { 
                        if t == "=" {
                            // Needs to consume rest of line
                            self.is_dec = true;
                            false
                        } else {
                            true
                        }
                    },
                    _ => { true }
                }
            },
            EnvParsingState::BeginOpeningCurly => {
                match token {
                    Token::Nothing(t, _) => { if t == "{" { self.env_parsing_state = EnvParsingState::BeginName } },
                    _ => {}
                };
                false
            },
            EnvParsingState::BeginName => {
                match token {
                    Token::Nothing(t, _) => { self.env_name = t.clone(); self.env_parsing_state = EnvParsingState::BeginClosingCurly; },
                    _ => {}
                }
                false },
            EnvParsingState::BeginClosingCurly => {
                match token {
                    Token::TexCommand(com, _) => { if com == "\\end" 
                    { if self.env_depth == 0 { self.env_parsing_state = EnvParsingState::EndOpeningCurly; } else { self.env_depth -= 1 }; }
                    else if com == "\\begin" { self.env_depth += 1; } },
                    _ => {}
                }
                false },
            EnvParsingState::EndOpeningCurly => { self.env_parsing_state = EnvParsingState::EndName; false },
            EnvParsingState::EndName => { 
                match token {
                    Token::Nothing(t, _) => { if t == &self.env_name { self.env_parsing_state = EnvParsingState::EndClosingCurly; }},
                    _ => {}
                }
                false },
            EnvParsingState::EndClosingCurly => { true },
            EnvParsingState::SquareBracketEquationEnv => { 
                match token {
                    Token::TexCommand(com, _) => { if com == "\\]" { self.next = true; } false },
                    _ => { false }
                }
            }
        }

    }

    fn parse (&mut self, tokens: TokenList, indentation_type: Option<IndentationType>, other_doc_locations: &mut OtherDocLocations) -> ParseResult {
        if self.env_parsing_state != EnvParsingState::NotEnv { // For environments
            //println!("{:#?}, {:#?}", self.env_parsing_state, tokens);
            // Strip tailing newline
            let edge_size = if self.env_parsing_state == EnvParsingState::SquareBracketEquationEnv { self.env_name = "equation".to_string(); 1 } else { 4 };
            let mut tokens = tokens;
            let children = node_list(tokens.clone(), edge_size, tokens.len() - edge_size, other_doc_locations)?;
            if self.env_name.clone() == "document" {
                // LiA adds the document macro implicitly, ignore existing document macro.
                return Ok((children, DocSection::Document))
            }
            return Ok((vec!{ Rc::new(TexEnvironment {
                name: self.env_name.clone(),
                args: vec![],
                children
            }) }, DocSection::Document))
        }

        
        let command = match &tokens[0] {
            Token::TexCommand(command, _) => { &command[1..] },
            _ => { println!("{:#?}", tokens); todo!() }
        }.to_string();
        if self.is_dec {
            let mut v = vec![
                Rc::new(TexCommand { command, args: vec![] }) as Rc<dyn Node>
            ];
            v.extend(node_list(tokens.clone(), 1, tokens.len(), other_doc_locations)?);
            return Ok((
                v,
                DocSection::Declarations
            ));
        }
        let section: DocSection = match command.as_str() {
            "usepackage" | "documentclass" | "usetikzlibrary" => { DocSection::Imports }
            "newcommand" | "renewcommand" | "DeclareMathOperator" | "definecolor" => { DocSection::Declarations }
            _ => { DocSection::Document }
        };
        let mut v = vec!{Rc::new( TexCommand {
            command,
            args: parse_args(&tokens, 1, tokens.len(), other_doc_locations)?
        }) as Rc<dyn Node>};
        if let Token::Newline = tokens.last().unwrap() {
            v.push(Rc::new( Text {
                text: "\n".to_string()
            }));
        }
        Ok((v, section))
        
    }
}