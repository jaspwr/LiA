use crate::parser_modules::variables::var_definition::LiaVarName;

#[derive(Debug, Clone)]
pub enum TypedValue {
    Number(f64),
    String(String)
}

impl TypedValue {
    pub fn stringify (&self) -> String {
        match self {
            TypedValue::Number(n) => n.to_string(),
            TypedValue::String(s) => s.clone()
        }
    }

    pub fn matches_declaration_type(&self, dec: &LiaVarName) -> bool {
        match dec {
            LiaVarName::Any(_) => true,
            LiaVarName::Number(_) => {
                match self {
                    TypedValue::Number(_) => true,
                    _ => false
                }
            },
            LiaVarName::String(_) => {
                match self {
                    TypedValue::String(_) => true,
                    _ => false
                }
            },
            _ => false
        }
    }

    pub fn type_name(&self) -> String {
        match self {
            TypedValue::Number(_) => "Number".to_string(),
            TypedValue::String(_) => "String".to_string()
        }
    }
}