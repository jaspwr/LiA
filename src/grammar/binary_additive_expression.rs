use std::rc::Rc;

use crate::ast::*;
use crate::typed_value::TypedValue;
use crate::at_expression::AtExpToken;

use super::token_from_list;

enum Operation {
    Add,
    Sub
}

pub struct BinaryAdditiveExpression {
    children: (AtExpToken, AtExpToken),
    operation: Operation
}

impl AstNode for BinaryAdditiveExpression {
    // TODO: refactor
    fn evaluate(&self, imported_values: &Vec<TypedValue>) -> Result<TypedValue, String> {
        if let AtExpToken::AstNode(left) = &self.children.0 {
            if let AtExpToken::AstNode(right) = &self.children.1 {
                let lhs = left.evaluate(imported_values)?;
                let rhs = right.evaluate(imported_values)?;
                match self.operation {
                    Operation::Add => {                
                        add(&lhs, &rhs)
                    },
                    Operation::Sub => {
                        sub(lhs, rhs)
                    }
                }
            } else {
                panic!("BinaryAdditionOperator::evaluate() called with non-AstNode token in right position.")
            }
        } else {
            panic!("BinaryAdditionOperator::evaluate() called with non-AstNode token in left position.")
        }
    }

    fn codegen(&self) -> String {
        if let AtExpToken::AstNode(left) = &self.children.0 {
            if let AtExpToken::AstNode(right) = &self.children.1 {
                match self.operation {
                    Operation::Add => format!("{} + {}", left.codegen(), right.codegen()),
                    Operation::Sub => format!("{} - {}", left.codegen(), right.codegen())
                }
            } else {
                panic!("BinaryAdditionOperator::codegen() called with non-AstNode token in right position.")
            }
        } else {
            panic!("BinaryAdditionOperator::codegen() called with non-AstNode token in left position.")
        }
    }
}

pub fn parse(tokens: &Vec<AtExpToken>, start: i32) -> Result<OpAstNode, String> {
    let add = token_from_list(tokens, start + 1).is_opertor_or_keyword("+");
    let sub = token_from_list(tokens, start + 1).is_opertor_or_keyword("-");
    if token_from_list(tokens, start).is_ast_node() &&
    ((add && !(token_from_list(tokens, start + 3).is_opertor_or_keyword("-") || (token_from_list(tokens, start - 1).is_opertor_or_keyword("-")))) 
    || sub) &&
    token_from_list(tokens, start + 2).is_ast_node() &&
    !(token_from_list(tokens, start - 1).is_opertor_or_keyword("*") 
        || token_from_list(tokens, start - 1).is_opertor_or_keyword("/")
        || token_from_list(tokens, start - 1).is_opertor_or_keyword("%")
        || token_from_list(tokens, start - 1).is_opertor_or_keyword("^")) &&
    !(token_from_list(tokens, start + 3).is_opertor_or_keyword("*") 
        || token_from_list(tokens, start + 3).is_opertor_or_keyword("/")
        || token_from_list(tokens, start + 3).is_opertor_or_keyword("%")
        || token_from_list(tokens, start + 3).is_opertor_or_keyword("^"))
    {

        Ok(Some((Rc::new(BinaryAdditiveExpression {
            children: (
                token_from_list(tokens, start),
                token_from_list(tokens, start + 2)
            ),
            operation: if add { Operation::Add } else { Operation::Sub }
        }), 3)))
    } else {
        Ok(None)
    }
}

fn sub(lhs: TypedValue, rhs: TypedValue) -> Result<TypedValue, String> {
    match lhs {
        TypedValue::Number(lhs) => {
            match rhs {
                TypedValue::Number(rhs) => Ok(TypedValue::Number(lhs - rhs)),
                _ => Err("Tried to subtract mismatched types in @() expression.".to_string())
            }
        },
        _ => Err("Tried to subtract a forbidden type in @() expression.".to_string())
    }
}

fn add(lhs: &TypedValue, rhs: &TypedValue) -> Result<TypedValue, String> {
    match lhs {
    TypedValue::Number(lhs) => {
            match rhs {
                TypedValue::Number(rhs) => Ok(TypedValue::Number(lhs + rhs)),
                _ => Err("Tried to add mismatched types in @() expression.".to_string())
            }
        },
        TypedValue::String(lhs) => {
            match rhs {
                TypedValue::String(rhs) => Ok(TypedValue::String(lhs.clone() + rhs)),
                _ => Err("Tried to add mismatched types in @() expression.".to_string())
            }
        },
    }
}