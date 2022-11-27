use std::rc::Rc;

use crate::parser_modules::variables::{ast::*, at_expression::{AtExpToken, TypedValue}};

use super::token_from_list;

enum Operation {
    Mul,
    Div,
    Mod
}
pub struct BinaryMultiplicativeExpression {
    children: (AtExpToken, AtExpToken),
    operation: Operation
}

impl AstNode for BinaryMultiplicativeExpression {
    // TODO: refactor
    fn evaluate(&self, imported_values: &Vec<TypedValue>) -> Result<TypedValue, String> {
        if let AtExpToken::AstNode(left) = &self.children.0 {
            if let AtExpToken::AstNode(right) = &self.children.1 {
                let lhs = left.evaluate(imported_values)?;
                let rhs = right.evaluate(imported_values)?;
                match lhs {
                    TypedValue::Number(lhs) => {
                        match rhs {
                            TypedValue::Number(rhs) => {
                                match self.operation {
                                    Operation::Mul => Ok(TypedValue::Number(lhs * rhs)),
                                    Operation::Div => Ok(TypedValue::Number(lhs / rhs)),
                                    Operation::Mod => Ok(TypedValue::Number(lhs % rhs))
                                }
                            },
                            _ => Err("Tried to multiply mismatched types in @() expression.".to_string())
                        }
                    },
                    _ => Err("Tried to multiply with imcompatible type in @() expression".to_string())
                }
            } else {
                panic!("BinaryAdditionOperator::evaluate() called with non-AstNode token in right position.")
            }
        } else {
            panic!("BinaryAdditionOperator::evaluate() called with non-AstNode token in left position.")
        }
    }
}

pub fn parse(tokens: &Vec<AtExpToken>, start: i32) -> Result<OpAstNode, String> {

    let mul = token_from_list(tokens, start + 1).is_opertor_or_keyword("*");
    let div = token_from_list(tokens, start + 1).is_opertor_or_keyword("/");
    let _mod = token_from_list(tokens, start + 1).is_opertor_or_keyword("%");
    if token_from_list(tokens, start).is_ast_node() &&
    (mul || div || _mod) && 
    token_from_list(tokens, start + 2).is_ast_node() {
        Ok(Some((Rc::new(BinaryMultiplicativeExpression {
            children: (
                token_from_list(tokens, start),
                token_from_list(tokens, start + 2)
            ),
            operation: if mul {
                Operation::Mul
            } else if div {
                Operation::Div
            } else {
                Operation::Mod
            }
        }), 3)))
    } else {
        Ok(None)
    }
    //Ok(None)
}