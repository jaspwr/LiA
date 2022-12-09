use std::rc::Rc;

use crate::ast::{AstNode, OpAstNode};
use crate::grammar::token_from_list;
use crate::typed_value::TypedValue;
use crate::at_expression::AtExpToken;

use super::check_either_side_for_opers;

pub struct TextNodePair {
    children: (AtExpToken, AtExpToken),
}

#[allow(unused)]
impl AstNode for TextNodePair {
    fn evaluate(&self, imported_values: &Vec<TypedValue>) -> Result<TypedValue, String> {
        Err("Can't evaluate text.".to_string())
    }

    fn codegen(&self) -> String {
        let value1 = match &self.children.0 {
            AtExpToken::AstNode(value) => value.codegen(),
            _ => "".to_string()
        };
        let value2 = match &self.children.1 {
            AtExpToken::AstNode(value) => value.codegen(),
            _ => "".to_string()
        };
        let has_space = !(value1.ends_with('_') || value2.starts_with('_') || value2.starts_with('{'));
        if has_space {
            format!("{} {}", value1, value2)
        } else {
            format!("{}{}", value1, value2)
        }
    }
}

pub fn parse(tokens: &Vec<AtExpToken>, start: i32) -> Result<OpAstNode, String> {
    if token_from_list(tokens, start).is_ast_node() &&
    token_from_list(tokens, start + 1).is_ast_node() &&
    !check_either_side_for_opers(tokens, start, 2, vec!["+", "-", "*", "/", "%", "^"])
    {
        Ok(Some((Rc::new(TextNodePair { children:
            (token_from_list(tokens, start).clone(), token_from_list(tokens, start + 1).clone())
        }), 2)))
    } else {
        Ok(None)
    }
}
