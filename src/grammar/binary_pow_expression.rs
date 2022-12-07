use std::rc::Rc;

use crate::ast::*;
use crate::at_expression::AtExpToken;
use crate::typed_value::TypedValue;

use super::token_from_list;

pub struct BinaryPowExpression {
    children: (AtExpToken, AtExpToken)
}

impl AstNode for BinaryPowExpression {
    fn evaluate(&self, imported_values: &Vec<TypedValue>) -> Result<TypedValue, String> {
        if let AtExpToken::AstNode(left) = &self.children.0 {
            if let AtExpToken::AstNode(right) = &self.children.1 {
                let lhs = left.evaluate(imported_values)?;
                let rhs = right.evaluate(imported_values)?;
                self.operate(lhs, rhs)
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
                format!("{}^{}", left.codegen(), right.codegen())
            } else {
                panic!("BinaryAdditionOperator::codegen() called with non-AstNode token in right position.")
            }
        } else {
            panic!("BinaryAdditionOperator::codegen() called with non-AstNode token in left position.")
        }
    }
}

impl BinaryPowExpression {
    fn operate(&self, lhs: TypedValue, rhs: TypedValue) -> Result<TypedValue, String> {
        match lhs {
            TypedValue::Number(lhs) => {
                match rhs {
                    TypedValue::Number(rhs) => {
                        Ok(TypedValue::Number(lhs.powf(rhs)))
                    },
                    _ => Err("Tried to raise an incompatible type to a power in @() expression.".to_string())
                }
            },
            _ => Err("Tried to raise to an incompatible type in @() expression".to_string())
        }
    }
}

pub fn parse(tokens: &Vec<AtExpToken>, start: i32) -> Result<OpAstNode, String> {
    if token_from_list(tokens, start).is_ast_node() &&
    token_from_list(tokens, start + 1).is_opertor_or_keyword("^") && 
    token_from_list(tokens, start + 2).is_ast_node() {
        Ok(Some((Rc::new(BinaryPowExpression {
            children: (
                token_from_list(tokens, start),
                token_from_list(tokens, start + 2)
            )
        }), 3)))
    } else {
        Ok(None)
    }
}