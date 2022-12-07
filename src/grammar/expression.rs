use std::rc::Rc;

use crate::ast::{AstNode, OpAstNode};
use crate::typed_value::TypedValue;
use crate::at_expression::AtExpToken;

use super::token_from_list;

enum BracketType {
    Curly,
    Round
}

pub struct Expression {
    value: AtExpToken,
    bracket_type: BracketType
}

impl AstNode for Expression {
    fn evaluate(&self, imported_values: &Vec<TypedValue>) -> Result<TypedValue, String> {
        if let AtExpToken::AstNode(node) = &self.value {
            Ok(node.evaluate(imported_values)?)
        } else {
            panic!("Expression::evaluate() called with non-AstNode token.")
        }
    }

    fn codegen(&self) -> String {
        if let AtExpToken::AstNode(node) = &self.value {
            match self.bracket_type {
                BracketType::Curly => format!("{{{}}}", node.codegen()),
                BracketType::Round => format!("\\left({}\\right)", node.codegen())
            }
        } else {
            panic!("Expression::codegen() called with non-AstNode token.")
        }
    }
}

pub fn parse(tokens: &Vec<AtExpToken>, start: i32) -> Result<OpAstNode, String> {
    let mut bracket_type: Option<BracketType> = None;
    if token_from_list(tokens, start).is_opertor_or_keyword("(") {
        bracket_type = Some(BracketType::Round);
    } else if token_from_list(tokens, start).is_opertor_or_keyword("{") {
        bracket_type = Some(BracketType::Curly);
    }
    if bracket_type.is_some() && token_from_list(tokens, start + 1).is_ast_node()
    {
        match bracket_type.unwrap() {
            BracketType::Round => {
                if token_from_list(tokens, start + 2).is_opertor_or_keyword(")") {
                    Ok(Some((Rc::new(Expression {
                        value: token_from_list(tokens, start + 1),
                        bracket_type: BracketType::Round
                    }), 3)))
                } else { Ok(None) }
            },
            BracketType::Curly => {
                if token_from_list(tokens, start + 2).is_opertor_or_keyword("}") {
                    Ok(Some((Rc::new(Expression {
                        value: token_from_list(tokens, start + 1),
                        bracket_type: BracketType::Curly
                    }), 3)))
                } else { Ok(None) }
            }
        }
    } else {
        Ok(None)
    }
}