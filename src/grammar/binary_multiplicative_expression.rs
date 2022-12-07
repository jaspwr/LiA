use std::rc::Rc;

use crate::ast::*;
use crate::at_expression::AtExpToken;
use crate::typed_value::TypedValue;

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
                match self.operation {
                    Operation::Mul => format!("{} \\times {}", left.codegen(), right.codegen()),
                    Operation::Div => format!("\\frac{{{}}}{{{}}}", left.codegen(), right.codegen()),
                    Operation::Mod => format!("{} \\mod {}", left.codegen(), right.codegen())
                }
            } else {
                panic!("BinaryAdditionOperator::codegen() called with non-AstNode token in right position.")
            }
        } else {
            panic!("BinaryAdditionOperator::codegen() called with non-AstNode token in left position.")
        }
    }
}

impl BinaryMultiplicativeExpression {
    fn operate(&self, lhs: TypedValue, rhs: TypedValue) -> Result<TypedValue, String> {
        match lhs {
            TypedValue::Number(lhs) => {
                match rhs {
                    TypedValue::Number(rhs) => {
                        self.evaluate_operation(lhs, rhs)
                    },
                    _ => Err("Tried to multiply mismatched types in @() expression.".to_string())
                }
            },
            _ => Err("Tried to multiply with incompatible type in @() expression".to_string())
        }
    }

    fn evaluate_operation(&self, lhs: f64, rhs: f64) -> Result<TypedValue, String> {
        match self.operation {
            Operation::Mul => Ok(TypedValue::Number(lhs * rhs)),
            Operation::Div => Ok(TypedValue::Number(lhs / rhs)),
            Operation::Mod => Ok(TypedValue::Number(lhs % rhs))
        }
    }
}

pub fn parse(tokens: &Vec<AtExpToken>, start: i32) -> Result<OpAstNode, String> {

    let mul = token_from_list(tokens, start + 1).is_opertor_or_keyword("*");
    let div = token_from_list(tokens, start + 1).is_opertor_or_keyword("/");
    let _mod = token_from_list(tokens, start + 1).is_opertor_or_keyword("%");
    if token_from_list(tokens, start).is_ast_node() &&
    (mul || div || _mod) &&
    token_from_list(tokens, start + 2).is_ast_node() &&
    !token_from_list(tokens, start - 1).is_opertor_or_keyword("^") &&
    !token_from_list(tokens, start + 3).is_opertor_or_keyword("^")
    {
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