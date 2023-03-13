use std::rc::Rc;

use crate::ast::{AstNode, OpAstNode};
use crate::at_expression::AtExpToken;
use crate::typed_value::TypedValue;

pub struct Literal {
    value: TypedValue,
}

#[allow(unused)]
impl AstNode for Literal {
    fn evaluate(&self, imported_values: &Vec<TypedValue>) -> Result<TypedValue, String> {
        Ok(self.value.clone())
    }

    fn codegen(&self) -> String {
        match &self.value {
            TypedValue::Number(n) => format!("{}", n),
            TypedValue::String(s) => format!("\"{}\"", s),
        }
    }
}

pub fn parse(tokens: &Vec<AtExpToken>, start: i32) -> Result<OpAstNode, String> {
    if let AtExpToken::Literal(value) = &tokens[start as usize] {
        match value {
            TypedValue::Number(n) => Ok(Some((
                Rc::new(Literal {
                    value: TypedValue::Number(*n),
                }),
                1,
            ))),
            TypedValue::String(s) => Ok(Some((
                Rc::new(Literal {
                    value: TypedValue::String(s.clone()),
                }),
                1,
            ))),
        }
    } else {
        Ok(None)
    }
}
