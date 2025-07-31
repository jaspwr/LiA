use std::rc::Rc;

use crate::{
    bracket_depth::BrackDepths,
    document::{Arg, ArgList, ArgType, DocSection, Node, NodeList, TexCommand, Text},
    feature_matrix::get_status_list,
    parse::{node_list, CompilerGlobals, IndentationType, NodeParser, ParseResult},
    token::*,
    tokenize::TokenList,
    utils::{count_whitespace, delta_bracket_depth, is_bracket, strip_all_whitespace, untokenise},
};

pub mod var_definition;
use crate::ast::Ast;
use crate::at_expression::*;
use crate::typed_value::TypedValue;
use var_definition::*;

#[derive(Default)]
pub struct LiaVariableParser {
    statement_type: Option<StatmentType>,
    terminated_by_newline: bool,
    consuming_rest_of_line: bool,
    trailing_whitespace: usize,
    curly_depth: i32,
}

#[derive(Clone)]
pub struct Function {
    name: String,
    args: Vec<LiaVarName>,
}

#[derive(Default)]
enum StatmentType {
    #[default]
    Read, // else
    Call,   // next_token = '('
    Assign, // next_token_no_white_space = '='
}

#[allow(unused)]
impl NodeParser for LiaVariableParser {
    fn is_opener(
        &mut self,
        tokens: &[Token],
        cursor: usize,
        identation: i32,
        other_doc_locations: &mut CompilerGlobals,
    ) -> bool {
        let token = &tokens[cursor];
        self.statement_type = None;
        self.consuming_rest_of_line = false;
        self.trailing_whitespace = 0;
        self.curly_depth = -1;
        match token {
            Token::LiaVariable(_, _) => true,
            _ => false,
        }
    }

    fn is_closer(&mut self, tokens: &[Token], cursor: usize, bracket_depths: &BrackDepths) -> bool {
        let token = &tokens[cursor];
        let next_token_no_white_space = &crate::utils::move_past_whitespace(tokens, cursor + 1).unwrap_or(Token::Newline);
        let next_token = &tokens[cursor + 1];

        if self.curly_depth == -1 {
            self.curly_depth = bracket_depths.curly;
        }
        if self.statement_type.is_none() {
            if let Token::Misc(next_token, _) = next_token {
                if next_token == "(" {
                    self.statement_type = Some(StatmentType::Call);
                }
            }
            if let Token::Misc(next_token_no_white_space, _) = next_token_no_white_space {
                if next_token_no_white_space == "=" {
                    self.statement_type = Some(StatmentType::Assign);
                }
            }
            if self.statement_type.is_none() {
                self.statement_type = Some(StatmentType::Read);
                return true; // Read type
            }
        } else {
            if self.consuming_rest_of_line {
                self.trailing_whitespace += 1;
                if let Token::Newline = token {
                    return true;
                }
                return false;
            } else {
                match self.statement_type {
                    Some(StatmentType::Read) => {
                        return true;
                    }
                    Some(StatmentType::Call) => return bracket_depths.round == 0,
                    Some(StatmentType::Assign) => {
                        return bracket_depths.curly == self.curly_depth
                            && match token {
                                Token::Newline => {
                                    self.terminated_by_newline = true;
                                    true
                                }
                                Token::Misc(t, _) => {
                                    if t == "}" {
                                        self.consuming_rest_of_line = true;
                                        self.terminated_by_newline = false;
                                        false
                                    } else {
                                        false
                                    }
                                }
                                _ => false,
                            }
                    }
                    None => {
                        return true;
                    }
                }
            }
        }
        false
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

        let command = match &tokens[0] {
            Token::LiaVariable(command, loc) => {
                    let command = &command[1..];
                    if command.len() == 0 {
                        return Err(
                            format!{"{} Invalid variable name \"{}\". Aborted.", loc.stringify(), command}
                        );
                    }
                    command
                },
            _ => { panic!("Should not be here.") }
        }.to_string();
        // TODO: Check for legal name

        match self.statement_type {
            Some(StatmentType::Read) => Ok((
                vec![Rc::new(TexCommand {
                    command,
                    args: vec![],
                })],
                DocSection::Document,
            )),
            Some(StatmentType::Call) => Ok((
                vec![Rc::new(TexCommand {
                    command: command.clone(),
                    args: split_call_args(
                        &tokens,
                        2,
                        tokens.len() - 1,
                        other_doc_locations,
                        is_defined_function(command, other_doc_locations.fucntions.clone()),
                    )?,
                })],
                DocSection::Document,
            )),
            Some(StatmentType::Assign) => {
                if command == "LIAVERSION" {
                    other_doc_locations.feature_status_list = get_status_list(
                        &strip_all_whitespace(untokenise(&tokens).split('=').last().unwrap()),
                    )?;
                    Ok((vec![], DocSection::Document))
                } else {
                    Ok((
                        vec![
                            parse_var_declaration(
                                command,
                                &tokens,
                                self.terminated_by_newline,
                                other_doc_locations,
                                self.trailing_whitespace,
                            )?,
                            Rc::new(Text {
                                text: "\n".to_string(),
                            }),
                        ],
                        DocSection::Declarations,
                    ))
                }
            }
            None => {
                todo!()
            }
        }
    }
}

fn is_defined_function(name: String, functions: Vec<Function>) -> Option<Function> {
    for f in &functions {
        if f.name == name {
            return Some(f.clone());
        }
    }
    None
}

fn split_call_args(
    tokens: &TokenList,
    start: usize,
    end: usize,
    other_doc_locations: &mut CompilerGlobals,
    function: Option<Function>,
) -> Result<Vec<Arg>, String> {
    let mut args: ArgList = Vec::new();
    let mut tokens_buffer: Vec<Token> = Vec::new();
    let mut str_args: Vec<Token> = Vec::new();
    for i in start..end {
        match &tokens[i] {
            Token::Misc(t, _) => {
                if t == "," {
                    let len = tokens_buffer.len();
                    //let whitespace = count_whitespace(&tokens_buffer, 0);
                    append_arg(
                        &mut args,
                        &tokens_buffer,
                        len,
                        other_doc_locations,
                        tokens,
                        &mut str_args,
                    )?;

                    tokens_buffer = Vec::new();
                } else {
                    tokens_buffer.push(tokens[i].clone());
                }
            }
            _ => {
                tokens_buffer.push(tokens[i].clone());
            }
        }
    }
    let len = tokens_buffer.len();
    if len > 0 {
        append_arg(
            &mut args,
            &tokens_buffer,
            len,
            other_doc_locations,
            tokens,
            &mut str_args,
        )?;
    }
    if function.is_some() {
        let function = function.unwrap();

        // TODO: Not this.
        let a: Vec<LiaVarName> = function
            .clone()
            .args
            .into_iter()
            .filter(|f| {
                if let LiaVarName::Lamda(_) = f {
                    true
                } else {
                    false
                }
            })
            .collect();
        let b: Vec<LiaVarName> = function
            .clone()
            .args
            .into_iter()
            .filter(|f| {
                if let LiaVarName::Lamda(_) = f {
                    false
                } else {
                    true
                }
            })
            .collect();
        let args_to_parse_in = to_typed_values(str_args)?;
        if args.len() != b.len() {
            return Err(format!(
                "{} Function {} takes {} arguments, but {} were given.",
                tokens[0].get_location().stringify(),
                function.name,
                b.len(),
                args.len()
            ));
        }
        for i in 0..args.len() {
            if !args_to_parse_in[i].matches_declaration_type(&b[i]) {
                return Err(format!(
                    "{} Recieved mismatched types for argument {} of function {}.",
                    tokens[0].get_location().stringify(),
                    i + 1,
                    function.name
                ));
            }
        }
        let mut errs: Vec<String> = Vec::new();
        a.into_iter().for_each(|f| {
            if let LiaVarName::Lamda(l) = f {
                args.push(Arg {
                    arg_type: ArgType::Curly,
                    arg: vec![Rc::new(Text {
                        text: match l.evaluate(&args_to_parse_in, "Failed for run @() expression.")
                        {
                            Ok(r) => r.stringify(),
                            Err(e) => {
                                errs.push(e);
                                "".to_string()
                            }
                        },
                    })],
                });
            }
        });
        if errs.len() > 0 {
            return Err(errs.join("\n"));
        }
    }
    Ok(args)
}

fn append_arg(
    args: &mut Vec<Arg>,
    tokens_buffer: &Vec<Token>,
    len: usize,
    other_doc_locations: &mut CompilerGlobals,
    tokens: TokenList,
    str_args: &mut Vec<Token>,
) -> Result<(), String> {
    args.push(Arg {
        arg_type: ArgType::Curly,
        arg: node_list(&tokens_buffer, 0, len, other_doc_locations)?,
    });
    let mut ws = count_whitespace(&tokens, 0);
    if ws > len - 1 {
        ws = len - 1
    }
    let a = &tokens_buffer[ws];
    str_args.push(a.clone());
    Ok(())
}

fn to_typed_values(args: Vec<Token>) -> Result<Vec<TypedValue>, String> {
    let mut err: Option<String> = None;
    let args = args
        .into_iter()
        .filter_map(|a| match a {
            Token::Misc(t, _) => Some(string_to_typed_value(t).unwrap()),
            _ => {
                err = Some(
                    format! {"{} Tried to pass an illegal argument.", a.get_location().stringify()},
                );
                None
            }
        })
        .collect();
    match err {
        Some(e) => Err(e),
        None => Ok(args),
    }
}

fn parse_var_declaration(
    command: String,
    tokens: &TokenList,
    terminated_by_newline: bool,
    other_doc_locations: &mut CompilerGlobals,
    trailing_whitespace: usize,
) -> Result<Rc<dyn Node>, String> {
    Ok(Rc::new(TexCommand {
        command: "newcommand".to_string(),
        args: match find_nothing_token(tokens, "=>") {
            None => {
                // There was no =>, so it is a const declaration.
                const_declaration_args(
                    command,
                    &tokens,
                    terminated_by_newline,
                    other_doc_locations,
                )?
            }
            Some(arrow_pos) => {
                let spl = tokens.split_at(arrow_pos);
                let mut lia_variables: Vec<LiaVarName> = parse_fn_declaration_lhs(spl.0)?;

                let function_inner: NodeList = parse_fn_declaration_rhs(
                    &spl.1[0..(spl.1.len() - trailing_whitespace)],
                    &mut lia_variables,
                    other_doc_locations,
                )?;
                let len = lia_variables.len();
                other_doc_locations.fucntions.push(Function {
                    name: command.clone(),
                    args: lia_variables,
                });
                function_declaration_args(command, len, function_inner)
            }
        },
    }))
}

fn function_declaration_args(command: String, argc: usize, fn_contents: NodeList) -> ArgList {
    vec![
        Arg {
            arg_type: ArgType::Curly,
            arg: vec![Rc::new(Text {
                text: format! {"\\{}", command},
            })],
        },
        Arg {
            arg_type: ArgType::Square,
            arg: vec![Rc::new(Text {
                text: argc.to_string(),
            })],
        },
        Arg {
            arg_type: ArgType::CurlyMultiline,
            arg: fn_contents,
        },
    ]
}

fn find_nothing_token(haystack: &TokenList, needle: &str) -> Option<usize> {
    for (i, t) in haystack.iter().enumerate() {
        if let Token::Misc(t, _) = t {
            if t == needle {
                return Some(i);
            }
        }
    }
    None
}

fn const_declaration_args(
    command: String,
    tokens: &TokenList,
    terminated_by_newline: bool,
    other_doc_locations: &mut CompilerGlobals,
) -> Result<ArgList, String> {
    let mut ret = vec![Arg {
        arg_type: ArgType::Curly,
        arg: vec![Rc::new(Text {
            text: format! {"\\{}", command},
        })],
    }];
    let equal_oper_pos = count_whitespace(tokens, 0);
    let content_pos = equal_oper_pos + count_whitespace(tokens, equal_oper_pos);
    ret.push(Arg {
        arg_type: ArgType::Curly,
        arg: node_list(
            tokens,
            content_pos,
            if terminated_by_newline {
                tokens.len() - 1
            } else {
                tokens.len()
            },
            other_doc_locations,
        )?,
    });
    Ok(ret)
}

fn parse_fn_declaration_lhs(tokens: TokenList) -> Result<Vec<LiaVarName>, String> {
    let mut ret: Vec<LiaVarName> = Vec::new();
    let mut brack_depth = BrackDepths::default();
    let mut sleep = 0;
    for i in 1..tokens.len() {
        if sleep > 0 {
            sleep -= 1;
            continue;
        }
        let t = &tokens[i];
        brack_depth += delta_bracket_depth(&t);
        let mut type_annotation = "Any".to_string();
        match extract_type_annotation(tokens, i) {
            None => {}
            Some((ta, s)) => {
                type_annotation = ta;
                sleep = s;
            }
        }
        match t {
            Token::LiaVariable(var, loc) => {
                ret.push(to_typed_var_name(
                    var[1..].to_string(),
                    type_annotation,
                    loc,
                )?);
            }
            Token::Misc(t, loc) => {
                if t != "," && t != "=" && !is_bracket(t.chars().next().unwrap()) {
                    ret.push(to_typed_var_name(t.clone(), type_annotation, loc)?);
                }
            }
            _ => {}
        }
    }
    if !brack_depth.is_zero() {
        return Err(
            format! {"{} Unbalanced brackets. Aborted.", tokens[0].get_location().stringify()},
        );
    }
    Ok(ret)
}

fn extract_type_annotation(tokens: TokenList, i: usize) -> Option<(String, usize)> {
    let ws = count_whitespace(tokens, i) + i;
    if let Token::Misc(t, _) = &tokens[if ws < tokens.len() {
        ws
    } else {
        tokens.len() - 1
    }] {
        if t == ":" {
            let ws = count_whitespace(tokens, ws) + ws;
            if let Token::Misc(t, _) = &tokens[if ws < tokens.len() {
                ws
            } else {
                tokens.len() - 1
            }] {
                return Some((t.to_string(), ws - i));
            }
        }
    }
    None
}

fn parse_fn_declaration_rhs(
    tokens: TokenList,
    lia_variables: &mut Vec<LiaVarName>,
    other_doc_locations: &mut CompilerGlobals,
) -> Result<NodeList, String> {
    let start = count_whitespace(&tokens, 2) + 2;
    let mut in_at_expression = false;
    let mut in_string_literal = false;
    let mut string_literal_buffer = String::new();
    let mut brack_depth = BrackDepths::default();
    let mut at_buf: Vec<Token> = Vec::new();
    let mut errors: Vec<String> = Vec::new();
    let mut did_error = false;
    let tokens: Vec<Token> = tokens
        .clone()
        .into_iter()
        .filter_map(|t| -> Option<Token> {
            if in_at_expression {
                brack_depth += delta_bracket_depth(&t);
                if in_string_literal {
                    match t {
                        Token::Misc(ref t, loc) => {
                            if t.ends_with('"') {
                                in_string_literal = false;
                                string_literal_buffer.push_str(t);
                                at_buf.push(Token::Misc(string_literal_buffer.clone(), *loc));
                                string_literal_buffer = String::new();
                            } else {
                                string_literal_buffer.push_str(t.as_str());
                            }
                        }
                        Token::Whitespace(ref ws) => {
                            string_literal_buffer.push_str(ws.as_str());
                        }
                        _ => {}
                    }
                    return None;
                } else {
                    return match t {
                        Token::Misc(t, loc) => {
                            if t.chars().into_iter().next() == Some('"')
                                && !(t.ends_with('"') && t.len() > 1)
                            {
                                in_string_literal = true;
                                string_literal_buffer.push_str(&t);
                                return None;
                            }
                            if t == ")" && brack_depth.round == 0 {
                                in_at_expression = false;
                                lia_variables.push(LiaVarName::Lamda(
                                    match parse_at_exprssion(&at_buf, lia_variables.clone()) {
                                        Ok(a) => a,
                                        Err(e) => {
                                            errors.push(e);
                                            did_error = true;
                                            Ast::default()
                                        }
                                    },
                                ));
                                at_buf = Vec::new();
                                Some(Token::Misc(format! {"#{}", lia_variables.len()}, *loc))
                            } else {
                                if !(t == "(" && brack_depth.round == 1) {
                                    at_buf.push(Token::Misc(t.clone(), *loc));
                                }
                                None
                            }
                        }
                        _ => None,
                    };
                }
            }
            match t {
                Token::LiaVariable(var, loc) => {
                    if var.len() == 1 {
                        in_at_expression = true;
                        return None;
                    }
                    for i in 0..lia_variables.len() {
                        if lia_variables[i].matches_name(&var[1..]) {
                            return Some(Token::Misc(format! {"#{}", i + 1}, *loc));
                        }
                    }
                    Some(Token::LiaVariable(var.clone(), *loc))
                }
                _ => Some(t.clone()),
            }
        })
        .collect();
    if did_error {
        errors.push("Failed to parse @() expression. Aborted.".to_string());
        return Err(errors.join("\n"));
    }

    // Bodge for when there is no whitespace between the curly bracket and the first token
    let mut start = start - 1;
    if let Token::Whitespace(_) = &tokens[start] {
        start += 1;
    } else if let Token::Misc(t, _) = &tokens[start] {
        if t == "{" {
            start += 1;
        }
    }

    let len = tokens.len();
    node_list(&tokens, start, len - 1, other_doc_locations)
}
