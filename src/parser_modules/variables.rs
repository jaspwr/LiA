use std::rc::Rc;

use crate::{tokeniser::{Token, TokenList}, hierachy_construction::{BrackDepths, NodeParser, node_list, IndentationType, ParseResult, DocSection}, hierarchy::{TexCommand, Arg, ArgType, ArgList, Text}, utils::{count_whitespace, format_error_string}};

#[derive(Default)]
pub struct LiaVariableParser {
    statement_type: Option<StatmentType>,
    terminated_by_newline: bool
}

#[derive(Default)]
enum StatmentType {
    #[default]
    Read, // else
    Call, // next_token = '('
    Assign // next_token_no_white_space = '='
}

#[allow(unused)]
impl NodeParser for LiaVariableParser {
    fn is_target(&mut self, token: &Token, identation: i32) -> bool {
        self.statement_type = None;
        match token {
            Token::LiaVariable(_, _) => { true },
            _ => { false }
        }
    }

    fn is_closer(&mut self, token: &Token, next_token: &Token, next_token_no_white_space: &Token, bracket_depths: &BrackDepths) -> bool {
        if self.statement_type.is_none() {
            if let Token::Nothing(next_token, _) = next_token {
                if next_token == "(" {
                    self.statement_type = Some(StatmentType::Call);
                }
            }
            if let Token::Nothing(next_token_no_white_space, _) = next_token_no_white_space {
                if next_token_no_white_space == "=" {
                    self.statement_type = Some(StatmentType::Assign);
                }
            }
            if self.statement_type.is_none() {
                self.statement_type = Some(StatmentType::Read);
                return true; // Read type
            }
        } else {
            match self.statement_type {
                Some(StatmentType::Read) => { return true; },
                Some(StatmentType::Call) => { return bracket_depths.round == 0 },
                Some(StatmentType::Assign) => { 
                    return bracket_depths.curly == 0
                    && match token {
                        Token::Newline => { self.terminated_by_newline = true; true }
                        Token::Nothing(t, _) => { if t == "}" { 
                            self.terminated_by_newline = false; true
                        } else { false }},
                        _ => { false }
                    } 
                },
                None => { return true; }
            }
        }
        false
    }

    fn parse (&mut self, tokens: TokenList, indentation_type: Option<IndentationType>) -> ParseResult {
        let command = match &tokens[0] {
            Token::LiaVariable(command, loc) => { 
                    let command = &command[1..];
                    if command.len() == 0 {
                        return Err(
                            format!{"{} Invalid varibale name \"{}\". Aborted.", loc.stringify(), command}
                        );
                    }
                    command
                },
            _ => { todo!() }
        }.to_string();
        // TODO: Check for legal name

        match self.statement_type {
            Some(StatmentType::Read) => {
                Ok((vec!{Rc::new( TexCommand {
                    command,
                    args: vec![]
                })}, DocSection::Document))
            },
            Some(StatmentType::Call) => {
                Ok((vec!{Rc::new( TexCommand {
                    command,
                    args: split_call_args(&tokens, 2, tokens.len() - 1)?
                })}, DocSection::Document))
            },
            Some(StatmentType::Assign) => {
                Ok((vec!{Rc::new( TexCommand {
                    command: "newcommand".to_string(),
                    args: newcommand_args(command, &tokens, self.terminated_by_newline)?
                }), Rc::new(Text { text: "\n".to_string() })}
                , DocSection::Declarations))
            },
            None => { todo!() }
        }

    }
}

fn newcommand_args(command: String, tokens: &TokenList, terminated_by_newline: bool) -> Result<ArgList, String> {
    let mut ret = vec![
        Arg {
            arg_type: ArgType::Curly,
            arg: vec![Rc::new(Text { text: format!{"\\{}", command} })]
        }
    ];
    let equal_oper_pos = count_whitespace(tokens, 0);
    let content_pos = equal_oper_pos + count_whitespace(tokens, equal_oper_pos);
    ret.push(Arg {
        arg_type: ArgType::Curly,
        arg: node_list(tokens.to_vec(), content_pos, if terminated_by_newline 
        { tokens.len() - 1} else { tokens.len() })?
    });
    Ok(ret)
}

fn split_call_args(tokens: &TokenList, start: usize, end: usize) -> Result<Vec<Arg>, String> {
    let mut args: ArgList = Vec::new();
    let mut tokens_buffer: TokenList = Vec::new();
    for i in start..end {
        match &tokens[i] {
            Token::Nothing(t, _) => {
                if t == "," {
                    let len = tokens_buffer.len();
                    //let whitespace = count_whitespace(&tokens_buffer, 0);
                    args.push(Arg {
                        arg_type: ArgType::Curly,
                        arg: node_list(tokens_buffer, 0, len)?
                    });
                    tokens_buffer = Vec::new();
                } else {
                    tokens_buffer.push(tokens[i].clone());
                }
            },
            _ => {
                tokens_buffer.push(tokens[i].clone());
            }
        }
    }
    let len = tokens_buffer.len();
    if len > 0 {
        args.push(Arg {
            arg_type: ArgType::Curly,
            arg: node_list(tokens_buffer, 0, len)?
        });
    }
    Ok(args)
}