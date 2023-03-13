use crate::parser_modules::variables::Ast;

#[derive(Clone)]
pub enum LiaVarName {
    Number(String),
    String(String),
    Size(String),
    Colour(String),
    Lamda(Ast),
    Any(String),
}

impl LiaVarName {
    pub fn matches_name(&self, name: &str) -> bool {
        match self {
            LiaVarName::Number(n) => n == name,
            LiaVarName::String(s) => s == name,
            LiaVarName::Size(s) => s == name,
            LiaVarName::Colour(c) => c == name,
            LiaVarName::Lamda(_) => false,
            LiaVarName::Any(a) => a == name,
        }
    }
}
