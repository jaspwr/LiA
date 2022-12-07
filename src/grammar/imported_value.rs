use std::rc::Rc;

use crate::ast::{AstNode, OpAstNode};
use crate::typed_value::TypedValue;
use crate::at_expression::AtExpToken;

pub struct ImportedValue {
    index: usize
}

impl AstNode for ImportedValue {
    fn evaluate(&self, imported_values: &Vec<TypedValue>) -> Result<TypedValue, String> {
        Ok(imported_values[self.index].clone())
    }

    fn codegen(&self) -> String {
        format!("imported_values[{}]", self.index)
    }
}

pub fn parse(tokens: &Vec<AtExpToken>, start: i32) -> Result<OpAstNode, String> {
    if let AtExpToken::Identifier(index) = tokens[start as usize] {
        Ok(Some((Rc::new(ImportedValue { index }), 1)))
    } else {
        Ok(None)
    }
}