use super::{ast::OpAstNode, at_expression::AtExpToken};

pub mod binary_additive_expression;
pub mod binary_multiplicative_expression;
pub mod binary_pow_expression;
pub mod expression;
pub mod imported_value;
pub mod literal;
pub mod text;
pub mod text_node_pair;
pub mod unary_additive_expression;
pub mod vector;

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
    if pos < 0 || pos >= tokens.len() as i32 {
        AtExpToken::Error
    } else {
        tokens[pos as usize].clone()
    }
}

pub fn check_either_side_for_opers(
    tokens: &Vec<AtExpToken>,
    pos: i32,
    len: i32,
    operators: Vec<&str>,
) -> bool {
    for operator in operators {
        if check_either_side_for_oper_single(tokens, pos, len, operator) {
            return true;
        }
    }
    false
}

pub fn check_either_side_for_oper_single(
    tokens: &Vec<AtExpToken>,
    pos: i32,
    len: i32,
    operator: &str,
) -> bool {
    token_from_list(tokens, pos - 1).is_opertor_or_keyword(operator)
        || token_from_list(tokens, pos + len).is_opertor_or_keyword(operator)
}
