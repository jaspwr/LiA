use std::rc::Rc;

use crate::ast::Ast;
use crate::at_expression::AtExpToken;
use crate::bracket_depth::BrackDepths;
use crate::hierachy_construction::{
    node_list, CompilerGlobals, IndentationType, NodeParser, ParseResult,
};
use crate::hierarchy::{DocSection, Node, TexEnvironment, Text};
use crate::token::*;
use crate::tokeniser::TokenList;
use crate::utils::format_error_string;

#[derive(Default)]
pub struct LiaEquation {}

static OPERATORS_AND_KEYWORDS: [&str; 15] = [
    "+", "-", "*", "/", "%", "?", ":", "(", ")", "{", "}", "^", ",", "[", "]",
];

#[allow(unused)]
impl NodeParser for LiaEquation {
    fn is_opener(
        &mut self,
        token: &Token,
        identation: i32,
        other_doc_locations: &mut CompilerGlobals,
    ) -> bool {
        match token {
            Token::Nothing(k, _) => k == "eq",
            _ => false,
        }
    }

    fn is_closer(
        &mut self,
        token: &Token,
        next_token: &Token,
        next_token_no_white_space: &Token,
        bracket_depths: &BrackDepths,
    ) -> bool {
        match token {
            Token::Nothing(t, _) => t == "}" && bracket_depths.curly == 0,
            _ => false,
        }
    }

    fn parse(
        &mut self,
        tokens: TokenList,
        indentation_type: Option<IndentationType>,
        other_doc_locations: &mut CompilerGlobals,
    ) -> ParseResult {
        let mut asterisk = false;
        let mut open_pos = 1;
        let len = tokens.len();
        while open_pos < len {
            if let Token::Whitespace(_) = tokens[open_pos] {
                open_pos += 1;
            } else if let Token::Nothing(t, loc) = &tokens[open_pos] {
                if t == "*" {
                    asterisk = true;
                    open_pos += 1;
                } else if t == "{" {
                    break;
                } else {
                    return format_error_string(
                        format! {"Unexpected token \"{}\" in equation statement.", t},
                        *loc,
                    );
                }
            } else {
                return format_error_string(
                    "Unexpected token in equation statement.".to_string(),
                    tokens[open_pos].get_location(),
                );
            }
        }

        let children = if contains_anything_meaningful(&tokens, open_pos + 1, len - 1)
            && other_doc_locations
                .feature_status_list
                .equation_statement_internal_syntax
                .is_supported()
        {
            vec![Rc::new(Text {
                text: Ast::construct(
                    &to_at_exp_tokens_for_equation(&tokens, open_pos + 1, len - 1)?,
                    0,
                    format!(
                        "{} Invalid syntax in equation statement",
                        tokens[0].get_location().stringify()
                    )
                    .as_str(),
                )?
                .codegen(),
            }) as Rc<dyn Node>]
        } else {
            node_list(tokens, open_pos + 1, len - 1, other_doc_locations)?
        };

        Ok((
            vec![Rc::new(TexEnvironment {
                name: if asterisk {
                    "[".to_string()
                } else {
                    "equation".to_string()
                },
                args: vec![],
                children,
            })],
            DocSection::Document,
        ))
    }
}

fn to_at_exp_tokens_for_equation(
    tokens: &TokenList,
    start: usize,
    end: usize,
) -> Result<Vec<AtExpToken>, String> {
    let mut at_exp_tokens = vec![];
    for i in start..end {
        let t_opt = tokenise(&tokens[i])?;
        if let Some(t) = t_opt {
            at_exp_tokens.push(t);
        }
    }
    Ok(at_exp_tokens)
}

fn tokenise(token: &Token) -> Result<Option<AtExpToken>, String> {
    match token {
        Token::Nothing(t, _) => {
            for op in OPERATORS_AND_KEYWORDS {
                if t == op {
                    return Ok(Some(AtExpToken::OperatorOrKeyword(t.to_string())));
                }
            }
            return Ok(Some(AtExpToken::Text(token.stringify())));
        }
        Token::TexCommand(_, _) => {
            return Ok(Some(AtExpToken::Text(token.stringify())));
        }
        Token::LiaKeyword(t, loc) => {
            return Err(format!("{} Unexpected keyword \"{}\" in equation statement. This will be supposed in future versions.", loc.stringify(), t));
        }
        Token::LiaVariable(_, _) => {
            return Err("Variables are not current supported in equation statements outside of functions. This will be supported in the future.".to_string());
        }
        _ => {
            return Ok(None);
        }
    }
}

fn contains_anything_meaningful(tokens: &TokenList, start: usize, end: usize) -> bool {
    for i in start..end {
        match &tokens[i] {
            Token::Whitespace(_) => {}
            Token::Newline => {}
            _ => {
                return true;
            }
        }
    }
    false
}
