use std::rc::Rc;

use crate::parser_modules::variables::{at_expression::{TypedValue, AtExpToken}, ast::{AstNode, OpAstNode}};

use super::token_from_list;

pub struct Expression {
    value: AtExpToken
}

impl AstNode for Expression {
    fn evaluate(&self, imported_values: &Vec<TypedValue>) -> Result<TypedValue, String> {
        if let AtExpToken::AstNode(node) = &self.value {
            Ok(node.evaluate(imported_values)?)
        } else {
            panic!("Expression::evaluate() called with non-AstNode token.")
        }
    }
}

pub fn parse(tokens: &Vec<AtExpToken>, start: i32) -> Result<OpAstNode, String> {
    if token_from_list(tokens, start).is_opertor_or_keyword("(") &&
    token_from_list(tokens, start + 1).is_ast_node() &&
    token_from_list(tokens, start + 2).is_opertor_or_keyword(")") {
        Ok(Some((Rc::new(Expression {
            value: token_from_list(tokens, start + 1)
        }), 3)))
    } else {
        Ok(None)
    }
}