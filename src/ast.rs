use std::rc::Rc;

use crate::at_expression::AtExpToken;

use super::grammar;
use super::typed_value::TypedValue;

#[derive(Clone)]
pub struct Ast {
    pub root_node: Option<DefAstNode>,
    pub imported_values_count: usize
}

impl Ast {
    pub fn default () -> Ast {
        Ast {
            root_node: None,
            imported_values_count: 0
        }
    }

    pub fn construct (tokens: &Vec<AtExpToken>, imported_values_count: usize, general_error_message: &str) -> Result<Ast, String> {
        let mut tokens = tokens.clone();
        const STALE: u32 = 200;
        let mut i = 0;
        while tokens.len() > 1 {
            let mut j = 0;
            while j < tokens.len() {
                let p = grammar::parse(&tokens, j as i32)?;
                match p {
                    Some((node, len)) => {
                        let mut new_tokens = Vec::new();
                        for k in 0..j {
                            new_tokens.push(tokens[k].clone());
                        }
                        new_tokens.push(AtExpToken::AstNode(node));
                        for k in j+len..tokens.len() {
                            new_tokens.push(tokens[k].clone());
                        }
                        tokens = new_tokens;
                        j += len;
                    },
                    None => {
                        j += 1;
                    }
                }
            }
            i += 1;
            if i > STALE {
                return Err(general_error_message.to_string());
            }
        }
        let root_node = match tokens[0] {
            AtExpToken::AstNode(ref node) => Some(node.clone()),
            _ => None
        };
        Ok(Ast {
            root_node,
            imported_values_count
        })
    }

    pub fn evaluate (&self, imported_values: &Vec<TypedValue>, invalid_arg_count_message: &str) -> Result<TypedValue, String> {
        if imported_values.len() != self.imported_values_count {
            return Err(format!("{} Expected {} arguments, got {}. Aborted",
            invalid_arg_count_message,
            self.imported_values_count, 
            imported_values.len()));
        }
        match self.root_node {
            Some(ref root) => Ok(root.evaluate(imported_values)?),
            None => { return Err("Attempted to evaluate an empty AST".to_string()); }
        }
    }

    pub fn codegen (&self) -> String {
        match self.root_node {
            Some(ref root) => root.codegen(),
            None => { return "".to_string(); }
        }
    }
}

pub type OpAstNode = Option<(Rc::<dyn AstNode>, usize)>;
pub type DefAstNode = Rc::<dyn AstNode>;

pub trait AstNode {
    fn evaluate(&self, imported_values: &Vec<TypedValue>) -> Result<TypedValue, String>;
    fn codegen(&self) -> String;
}