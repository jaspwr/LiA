use std::rc::Rc;

use crate::ast::*;
use crate::typed_value::TypedValue;
use crate::at_expression::AtExpToken;

use super::token_from_list;

enum Operation {
    Plus,
    Minus
}

pub struct UnaryAdditiveExpression {
    child: AtExpToken,
    operation: Operation
}

impl AstNode for UnaryAdditiveExpression {
    fn evaluate(&self, imported_values: &Vec<TypedValue>) -> Result<TypedValue, String> {
        if let AtExpToken::AstNode(child) = &self.child {
            let child_value = child.evaluate(imported_values)?;
            match self.operation {
                Operation::Plus => {                
                    Ok(child_value)
                    // Maybe do something funny like parse strings like javascript
                },
                Operation::Minus => {
                    match child_value {
                        TypedValue::Number(i) => Ok(TypedValue::Number(-i)),
                        _ => Err(format!("Cannot use unary minus on type {}.", child_value.type_name()))
                    }
                }
            }
        } else {
            panic!("UnaryAdditiveExpression::evaluate() called with non-AstNode token in left position.")
        }
    }

    fn codegen(&self) -> String {
        if let AtExpToken::AstNode(child) = &self.child {
            match self.operation {
                Operation::Plus => {
                    format!("+{}", child.codegen())
                },
                Operation::Minus => {
                    format!("-{}", child.codegen())
                }
            }
        } else { panic!("UnaryAdditiveExpression::codegen() called with non-AstNode token in left position.")}
    }
}

pub fn parse(tokens: &Vec<AtExpToken>, start: i32) -> Result<OpAstNode, String> {
    let plus = token_from_list(tokens, start).is_opertor_or_keyword("+");
    let minus = token_from_list(tokens, start).is_opertor_or_keyword("-");
    if (plus || minus) &&
    token_from_list(tokens, start + 1).is_ast_node() &&
    !token_from_list(tokens, start - 1).is_ast_node()
    {

        Ok(Some((Rc::new(UnaryAdditiveExpression {
            child: token_from_list(tokens, start + 1),
            operation: if plus { Operation::Plus } else { Operation::Minus }
        }), 2)))
    } else {
        Ok(None)
    }
}