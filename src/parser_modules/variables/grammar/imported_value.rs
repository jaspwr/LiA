use std::rc::Rc;

use crate::parser_modules::variables::{at_expression::{TypedValue, AtExpToken}, ast::{AstNode, OpAstNode}};

pub struct ImportedValue {
    index: usize
}

impl AstNode for ImportedValue {
    fn evaluate(&self, imported_values: &Vec<TypedValue>) -> Result<TypedValue, String> {
        Ok(imported_values[self.index].clone())
    }
}

pub fn parse(tokens: &Vec<AtExpToken>, start: i32) -> Result<OpAstNode, String> {
    if let AtExpToken::Identifier(index) = tokens[start as usize] {
        Ok(Some((Rc::new(ImportedValue { index }), 1)))
    } else {
        Ok(None)
    }
}