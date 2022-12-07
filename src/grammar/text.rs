use std::rc::Rc;

use crate::ast::{AstNode, OpAstNode};
use crate::typed_value::TypedValue;
use crate::at_expression::AtExpToken;

pub struct AstText {
    value: String
}

#[allow(unused)]
impl AstNode for AstText {
    fn evaluate(&self, imported_values: &Vec<TypedValue>) -> Result<TypedValue, String> {
        Err("Can't evaluate text.".to_string())
    }

    fn codegen(&self) -> String {
        self.value.clone()
    }
}

pub fn parse(tokens: &Vec<AtExpToken>, start: i32) -> Result<OpAstNode, String> {
    if let AtExpToken::Text(value) = &tokens[start as usize] {
        Ok(Some((Rc::new(AstText { value: value.clone() }), 1)))
    } else {
        Ok(None)
    }
}