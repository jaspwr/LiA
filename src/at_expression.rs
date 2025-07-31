use crate::token::*;

use super::ast::*;
use super::typed_value::TypedValue;
use crate::parser_modules::variables::var_definition::LiaVarName;

static OPERATORS_AND_KEYWORDS: [&str; 15] = [
    "+", "-", "*", "/", "%", "?", ":", "(", ")", "{", "}", "^", ",", "[", "]",
];

pub fn parse_at_exprssion(
    tokens: &Vec<Token>,
    lia_variables: Vec<LiaVarName>,
) -> Result<Ast, String> {
    let lia_variables = lia_variables
        .into_iter()
        .filter(|v| {
            if let LiaVarName::Lamda(_) = v {
                false
            } else {
                true
            }
        })
        .collect();
    let mut errors: Vec<String> = Vec::new();
    let mut did_error = false;
    let tokens: Vec<AtExpToken> = tokens
        .iter()
        .map(|t| match AtExpToken::tokenise(t, &lia_variables) {
            Ok(t) => t,
            Err(e) => {
                errors.push(e);
                did_error = true;
                AtExpToken::Error
            }
        })
        .collect();
    if did_error {
        return Err(errors.join("\n").to_string());
    }
    if tokens.is_empty() {
        return Err("Found empty @() expression.".to_string());
    }
    let ast = Ast::construct(
        &tokens,
        lia_variables.len(),
        "Could not parse @() expression",
    )?;
    Ok(ast)
}

#[derive(Clone)]
pub enum AtExpToken {
    Identifier(usize),
    Literal(TypedValue),
    OperatorOrKeyword(String),
    AstNode(DefAstNode),
    Text(String),
    Error,
}

impl AtExpToken {
    pub fn is_ast_node(&self) -> bool {
        match self {
            AtExpToken::AstNode(_) => true,
            _ => false,
        }
    }

    pub fn is_opertor_or_keyword(&self, op: &str) -> bool {
        match self {
            AtExpToken::OperatorOrKeyword(_op) => op == _op,
            _ => false,
        }
    }
}

impl AtExpToken {
    fn tokenise(
        token: &Token,
        imported_value_names: &Vec<LiaVarName>,
    ) -> Result<AtExpToken, String> {
        match token {
            Token::Misc(t, loc) => {
                let first_char = t.chars().next().unwrap();
                if first_char.is_numeric() {
                    return parse_numerical_literal(t.clone(), *loc);
                } else if first_char == '"' {
                    return Ok(AtExpToken::Literal(TypedValue::String(
                        t[1..t.len() - 1].to_string(),
                    )));
                }
                for op in OPERATORS_AND_KEYWORDS {
                    if t == op {
                        return Ok(AtExpToken::OperatorOrKeyword(t.to_string()));
                    }
                }
            }
            _ => panic!("Unexpected token in @() expression."),
        }
        Ok(AtExpToken::Identifier(get_imported_value_index(
            token.clone(),
            imported_value_names,
        )?))
    }
}

pub fn string_to_typed_value(s: String) -> Result<TypedValue, String> {
    if let Ok(n) = s.parse::<f64>() {
        Ok(TypedValue::Number(n))
    } else {
        Ok(TypedValue::String(s))
    }
}

pub fn parse_numerical_literal(s: String, loc: Location) -> Result<AtExpToken, String> {
    match s.parse::<f64>() {
        Ok(n) => Ok(AtExpToken::Literal(TypedValue::Number(n))),
        Err(_) => Err(format!(
            "{} Invalid syntanx in @(), \"{}\".",
            loc.stringify(),
            s
        )),
    }
}

fn get_imported_value_index(
    token: Token,
    imported_value_names: &Vec<LiaVarName>,
) -> Result<usize, String> {
    match token {
        Token::Misc(t, loc) => {
            for i in 0..imported_value_names.len() {
                if imported_value_names[i].matches_name(t.as_str()) {
                    return Ok(i);
                }
            }
            Err(format!(
                "{} No value with name \"{}\" in @() found.",
                loc.stringify(),
                t
            ))
        }
        _ => {
            panic!("Unexpected token in @() expression.")
        }
    }
}

pub fn to_typed_var_name(
    name: String,
    type_annotation: String,
    location: &Location,
) -> Result<LiaVarName, String> {
    match type_annotation.as_str() {
        "Number" | "num" => Ok(LiaVarName::Number(name)),
        "String" | "txt" => Ok(LiaVarName::String(name)),
        "Size" | "sz" => Ok(LiaVarName::Size(name)),
        "Colour" | "Color" | "col" => Ok(LiaVarName::Colour(name)),
        "Lamda" | "fn" | "λ" => Ok(LiaVarName::Lamda(Ast::default())),
        "Any" => Ok(LiaVarName::Any(name)),
        _ => {
            Err(format! {"{} Unknown type \"{}\". Aborted.", location.stringify(), type_annotation})
        }
    }
}
