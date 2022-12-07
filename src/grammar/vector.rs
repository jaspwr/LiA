use std::rc::Rc;

use crate::ast::{AstNode, OpAstNode};
use crate::typed_value::TypedValue;
use crate::at_expression::AtExpToken;

use super::token_from_list;

pub struct Vector_ {
    children: Vec<AtExpToken>,
}

impl AstNode for Vector_ {
    fn evaluate(&self, imported_values: &Vec<TypedValue>) -> Result<TypedValue, String> {
        Err("Can't evaluate vectors. Vector arithmetic yet supported.".to_string())
    }

    fn codegen(&self) -> String {
        let mut code = String::new();
        code.push_str("\\begin{pmatrix} ");
        code.push_str(codegen_row(&self.children).as_str());
        code.push_str(" \\end{pmatrix}");
        code
    }
}

pub fn parse(tokens: &Vec<AtExpToken>, start: i32) -> Result<OpAstNode, String> {
    let mut children: Vec<AtExpToken> = Vec::new();
    if token_from_list(tokens, start).is_opertor_or_keyword("[") {
        let mut end = start + 1;
        if token_from_list(tokens, start + 1).is_opertor_or_keyword("]") {
            return Ok(Some((Rc::new(Vector_ {
                children
            }), 2)));
        }
        let len = tokens.len() as i32;
        while end < len {
            if token_from_list(tokens, end).is_ast_node() {
                children.push(token_from_list(tokens, end));
                end += 1;
            } else {
                return Ok(None);
            }
            if token_from_list(tokens, end).is_opertor_or_keyword(",") {
                end += 1;
            } else if token_from_list(tokens, end).is_opertor_or_keyword("]") {
                return Ok(Some((Rc::new(Vector_ {
                    children
                }), (end - start + 1) as usize)));
            } else {
                return Ok(None);
            }
        }
        Ok(None)
    } else {
        Ok(None)
    }
}

fn codegen_row (children: &Vec<AtExpToken>) -> String {
    let mut code = String::new();
    let mut first = true;
    let mut dont_add_ampersand = false;
    for child in children {
        if !first {
            if !dont_add_ampersand {
                code.push_str(" & ");
            } else {
                dont_add_ampersand = false;
            }
        }
        if let AtExpToken::AstNode(child) = child {
            let mut child_code = child.codegen();
            if child_code.starts_with("\\begin{pmatrix}") {
                if !first {
                    code.push_str(" \\\\ ");
                }
                let a = child_code[16..child_code.len()-14].to_string();
                child_code = a.clone();
                dont_add_ampersand = true;
            }
            code.push_str(&child_code);
        }
        first = false;
    }
    code
}