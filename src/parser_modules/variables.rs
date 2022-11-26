use std::rc::Rc;

use crate::{tokeniser::{Token, TokenList}, hierachy_construction::{BrackDepths, NodeParser, node_list, IndentationType, ParseResult, DocSection, OtherDocLocations}, hierarchy::{TexCommand, Arg, ArgType, ArgList, Text, Node, NodeList}, utils::{count_whitespace, delta_bracket_depth, is_bracket}};

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

    fn parse (&mut self, tokens: TokenList, indentation_type: Option<IndentationType>, other_doc_locations: &mut OtherDocLocations) -> ParseResult {
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
                    args: split_call_args(&tokens, 2, tokens.len() - 1, other_doc_locations)?
                })}, DocSection::Document))
            },
            Some(StatmentType::Assign) => {
                Ok((vec!{parse_var_declaration(command, &tokens,  self.terminated_by_newline, other_doc_locations)?, Rc::new(Text { text: "\n".to_string() })}
                , DocSection::Declarations))
            },
            None => { todo!() }
        }

    }
}

fn split_call_args(tokens: &TokenList, start: usize, end: usize, other_doc_locations: &mut OtherDocLocations) -> Result<Vec<Arg>, String> {
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
                        arg: node_list(tokens_buffer, 0, len, other_doc_locations)?
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
            arg: node_list(tokens_buffer, 0, len, other_doc_locations)?
        });
    }
    Ok(args)
}

fn parse_var_declaration(command: String, tokens: &TokenList,
    terminated_by_newline: bool, other_doc_locations: &mut OtherDocLocations) -> Result<Rc<dyn Node>, String> {
    Ok(Rc::new( TexCommand {
        command: "newcommand".to_string(),
        args: match find_nothing_token(tokens, "=>") {
            None => {
                // There was no =>, so it is a econst declaration.
                const_declaration_args(command, &tokens, terminated_by_newline, other_doc_locations)?
            },
            Some(arrow_pos) => {
                let spl = tokens.split_at(arrow_pos);
                let mut lia_variables: Vec<String> = parse_fn_declaration_lhs(spl.0.to_vec())?;
                let function_inner: NodeList = parse_fn_declaration_rhs(spl.1.to_vec(),&mut lia_variables, other_doc_locations)?;
                function_declaration_args(command, lia_variables.len(), function_inner)
            }
        }
    }))
}

fn find_nothing_token (haystack: &TokenList, needle: &str) -> Option<usize> {
    for (i, t) in haystack.iter().enumerate() {
        if let Token::Nothing(t, _) = t {
            if t == needle {
                return Some(i);
            }
        }
    }
    None
}

fn const_declaration_args(command: String, tokens: &TokenList, terminated_by_newline: bool, other_doc_locations: &mut OtherDocLocations) -> Result<ArgList, String> {
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
        { tokens.len() - 1} else { tokens.len() }, other_doc_locations)?
    });
    Ok(ret)
}

fn function_declaration_args (command: String, argc: usize, fn_contents: NodeList) -> ArgList {
    vec![
        Arg {
            arg_type: ArgType::Curly,
            arg: vec![Rc::new(Text { text: format!{"\\{}", command} })]
        },
        Arg {
            arg_type: ArgType::Square,
            arg: vec![Rc::new(Text { text: argc.to_string() })]
        },
        Arg {
            arg_type: ArgType::CurlyMultiline,
            arg: fn_contents
        }
    ]
}

fn parse_fn_declaration_lhs(tokens: TokenList) -> Result<Vec<String>, String> {
    let mut ret: Vec<String> = Vec::new();
    let mut brack_depth = BrackDepths::default();
    for i in 1..tokens.len() {
        let t = &tokens[i];
        brack_depth += delta_bracket_depth(&t);
        match t {
            Token::LiaVariable(var, _) => {
                ret.push(var[1..].to_string());
            },
            Token::Nothing(t, _) => {
                if t != "," && t != "=" && !is_bracket(t.chars().next().unwrap()) {
                    ret.push(t.clone());
                }
            },
            _ => {}
        }
    }
    if !brack_depth.is_zero() {
        return Err(format!{"{} Unbalanced brackets. Aborted.", tokens[0].get_location().stringify()});
    }
    Ok(ret)
}

fn parse_fn_declaration_rhs(tokens: TokenList, lia_variables: &mut Vec<String>, 
    other_doc_locations: &mut OtherDocLocations) -> Result<NodeList, String> {
    let len = tokens.len();
    let start = count_whitespace(&tokens, 2) + 2;
    let tokens = tokens.into_iter().map(|t| {
        match t {
            Token::LiaVariable(var, loc) => {
                for i in 0..lia_variables.len() {
                    if lia_variables[i] == var[1..] {
                        return Token::Nothing(
                            format!{"#{}", i + 1},
                            loc
                        );
                    }
                }
                Token::LiaVariable(var, loc)
            },
            _ => t
        }
    }).collect();
    node_list(tokens, start, len - 1, other_doc_locations)
}