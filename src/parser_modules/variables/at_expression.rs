use std::{rc::Rc, error};

use crate::tokeniser::{Token, Location};

use super::LiaVarName;
use super::ast::*;

pub fn parse_at_exprssion (tokens: &Vec<Token>, lia_variables: Vec<LiaVarName>) -> Result<Ast, String> {
    let lia_variables = lia_variables.into_iter().filter(|v| {
        if let LiaVarName::Lamda(_) = v {
            false
        } else {
            true
        }
    }).collect();
    let mut errors: Vec<String> = Vec::new();
    let mut did_error = false;
    let tokens: Vec<AtExpToken> = tokens.into_iter()
    .map(|t| {
        match AtExpToken::tokenise(t, &lia_variables) {
            Ok(t) => t,
            Err(e) => {
                errors.push(e);
                did_error = true;
                AtExpToken::Error
            }
        }
    }).collect();
    if did_error {
        return Err(errors.join("\n").to_string());
    }
    let ast = Ast::construct(&tokens, lia_variables.len())?;
    Ok(ast)
}


#[derive(Debug, Clone)]
pub enum TypedValue {
    Number(f64),
    String(String)
}

impl TypedValue {
    pub fn stringify (&self) -> String {
        match self {
            TypedValue::Number(n) => n.to_string(),
            TypedValue::String(s) => s.clone()
        }
    }

    pub fn matches_declaration_type(&self, dec: &LiaVarName) -> bool {
        match dec {
            LiaVarName::Any(_) => true,
            LiaVarName::Number(_) => {
                match self {
                    TypedValue::Number(_) => true,
                    _ => false
                }
            },
            LiaVarName::String(_) => {
                match self {
                    TypedValue::String(_) => true,
                    _ => false
                }
            },
            _ => false
        }
    }
}

#[derive(Clone)]
pub enum AtExpToken {
    Identifier(usize),
    Literal(TypedValue),
    OperatorOrKeyword(String),
    AstNode(DefAstNode),
    Error
}

impl AtExpToken {
    pub fn is_ast_node (&self) -> bool {
        match self {
            AtExpToken::AstNode(_) => true,
            _ => false
        }
    }

    pub fn is_opertor_or_keyword (&self, op: &str) -> bool {
        match self {
            AtExpToken::OperatorOrKeyword(_op) => op == _op,
            _ => false
        }
    }
}

static OPERATORS_AND_KEYWORDS: [&str; 9] = ["+", "-", "*", "/", "%", "?", ":", "(", ")"];

pub fn op(s: &str) -> usize {
    // Hopefully LLVM will optimise this out
    let mut i = 0;
    while OPERATORS_AND_KEYWORDS[i] != s {
        i += 1;
        if i > OPERATORS_AND_KEYWORDS.len() {
            panic!("Operator not found");
        }
    }
    i
}

impl AtExpToken {
    fn tokenise(token: &Token, imported_value_names: &Vec<LiaVarName>) -> Result<AtExpToken, String> {
        match token {
            Token::Nothing(t, loc) => {
                let first_char = t.chars().next().unwrap();
                if first_char.is_numeric() {
                    return Ok(parse_numerical_literal(t.clone(), *loc)?)
                } else if first_char == '"' {
                    return Ok(AtExpToken::Literal(
                        TypedValue::String(t[1..t.len()-1].to_string())
                    ));
                }
                for op in OPERATORS_AND_KEYWORDS {
                    if t == op {
                        return Ok(AtExpToken::OperatorOrKeyword(t.to_string()));
                    }
                }
            },
            _ => panic!("Unexpected token in @() expression.")
        }
        return Ok(AtExpToken::Identifier(get_imported_value_index(token.clone(), imported_value_names)?));
    }
}

pub fn string_to_typed_value(s: String) -> Result<TypedValue, String> {
    if let Ok(n) = s.parse::<f64>() {
        Ok(TypedValue::Number(n))
    } else {
        Ok(TypedValue::String(s))
    }
}

pub fn parse_numerical_literal (s: String, loc: Location) -> Result<AtExpToken, String> {
    match s.parse::<f64>() {
        Ok(n) => Ok(AtExpToken::Literal(TypedValue::Number(n))),
        Err(_) => Err(format!("{} Invalid syntanx in @(), \"{}\".", loc.stringify(), s))
    }
}

fn get_imported_value_index(token: Token, imported_value_names: &Vec<LiaVarName>) -> Result<usize, String> {
    match token {
        Token::Nothing(t, loc) => {
            for i in 0..imported_value_names.len() {
                if imported_value_names[i].matches_name(t.as_str()) {
                    return Ok(i);
                }
            }
            Err(format!("{} No value with name \"{}\" in @() found.", loc.stringify(), t))
        },
        _ => { panic!("Unexpected token in @() expression.") }
    }
}