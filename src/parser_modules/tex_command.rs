use std::rc::Rc;

use crate::{tokeniser::{Token, TokenList}, hierachy_construction::{BrackDepths, NodeParser, IndentationType, ParseResult, DocSection, node_list}, hierarchy::{TexCommand, TexEnvironment}, utils::parse_args};

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
}

#[derive(Default)]
pub struct TexCommandParser {
    env_parsing_state: EnvParsingState,
    env_name: String
}

#[allow(unused)]
impl NodeParser for TexCommandParser {
    fn is_target(&mut self, token: &Token, identation: i32) -> bool {
        self.env_parsing_state = EnvParsingState::NotEnv;
        match token {
            Token::TexCommand(com, _) => { 
                if com == "\\begin" { println!("{:?}", token); self.env_parsing_state = EnvParsingState::BeginOpeningCurly; }; 
                println!("{:?}", self.env_parsing_state);
                
                true },
            _ => { false }
        }
    }

    fn is_closer(&mut self, token: &Token, next_token: &Token, next_token_no_white_space: &Token, bracket_depths: &BrackDepths) -> bool {
        match self.env_parsing_state {
            EnvParsingState::NotEnv => {
                println!("FFFssF");
                bracket_depths.curly == 0 && bracket_depths.square == 0
                && match next_token {
                    Token::Nothing(t, _) => { t != "{" && t != "[" },
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
                    Token::TexCommand(com, _) => { if com == "\\end" { self.env_parsing_state = EnvParsingState::EndOpeningCurly; }},
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
        }

    }

    fn parse (&mut self, tokens: TokenList, indentation_type: Option<IndentationType>) -> ParseResult {
        println!("{}", self.env_name);
        println!("{:?}", self.env_parsing_state);

        if self.env_parsing_state == EnvParsingState::NotEnv {
            let command = match &tokens[0] {
                Token::TexCommand(command, _) => { &command[1..] },
                _ => { todo!() }
            }.to_string();
            if command == "document" {
                // LiA adds the document macro implicitly, ignore existing document macro.
                return Ok((vec![], DocSection::Imports))
            }
            let section: DocSection = match command.as_str() {
                "usepackage" => { DocSection::Imports }
                "newcommand" | "renewcommand" | "DeclareMathOperator" | "definecolor" => { DocSection::Declarations }
                _ => { DocSection::Document }
            };
            Ok((vec!{Rc::new( TexCommand {
                command,
                args: parse_args(&tokens, 1, tokens.len())?
            })}, section))
        } else {
            Ok((vec!{ Rc::new(TexEnvironment {
                name: self.env_name.clone(),
                args: vec![],
                children: node_list(tokens.clone(), 5, tokens.len() - 5)?
            }) }, DocSection::Document))
        }
    }
}