use super::{at_expression::AtExpToken, ast::OpAstNode};

pub mod binary_additive_expression;
pub mod binary_multiplicative_expression;
pub mod literal;
pub mod imported_value;
pub mod expression;
pub mod unary_additive_expression;
pub mod binary_pow_expression;
pub mod vector;
pub mod text;
pub mod text_node_pair;

pub fn parse(tokens: &Vec<AtExpToken>, start: i32) -> Result<OpAstNode, String> {
    if let Some(r) = binary_additive_expression::parse(tokens, start)? {
        Ok(Some(r))
    } else if let Some(r) = binary_pow_expression::parse(tokens, start)? {
        Ok(Some(r))
    } else if let Some(r) = binary_multiplicative_expression::parse(tokens, start)? {
        Ok(Some(r))
    } else if let Some(r) = literal::parse(tokens, start)? {
        Ok(Some(r))
    } else if let Some(r) = imported_value::parse(tokens, start)? {
        Ok(Some(r))
    } else if let Some(r) = expression::parse(tokens, start)? {
        Ok(Some(r))
    } else if let Some(r) = unary_additive_expression::parse(tokens, start)? {
        Ok(Some(r))
    } else if let Some(r) = vector::parse(tokens, start)? {
        Ok(Some(r))
    } else if let Some(r) = text::parse(tokens, start)? {
        Ok(Some(r))
    } else if let Some(r) = text_node_pair::parse(tokens, start)? {
        Ok(Some(r))
    } else {
        Ok(None)
    }
}

pub fn token_from_list(tokens: &Vec<AtExpToken>, pos: i32) -> AtExpToken {
    if pos < 0 ||  pos >= tokens.len() as i32 {
        AtExpToken::Error
    } else {
        tokens[pos as usize].clone()
    }
}