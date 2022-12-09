use std::rc::Rc;

use crate::ast::{AstNode, OpAstNode};
use crate::typed_value::TypedValue;
use crate::at_expression::AtExpToken;

pub struct AstText {
    value: String
}

struct TxtRpl {
    text: &'static str,
    replacment: &'static str
}

static REPLACMENTS: [TxtRpl; 30] = [
    TxtRpl { text: "<->", replacment: "\\leftrightarrow" },
    TxtRpl { text: "<=>", replacment: "\\leftrightharpoons" },
    TxtRpl { text: "<=", replacment: "\\le" },
    TxtRpl { text: ">=", replacment: "\\ge" },
    TxtRpl { text: "+-", replacment: "\\pm" },
    TxtRpl { text: "-+", replacment: "\\mp" },
    TxtRpl { text: "=>", replacment: "\\implies" },
    TxtRpl { text: "!=", replacment: "\\ne" },
    TxtRpl { text: "->", replacment: "\\rightarrow" },
    TxtRpl { text: "<-", replacment: "\\leftarrow" },
    TxtRpl { text: "^^", replacment: "\\uparrow" },
    TxtRpl { text: "~==", replacment: "\\cong" },
    TxtRpl { text: "~=", replacment: "\\simeq" },
    TxtRpl { text: "~~", replacment: "\\approx" },
    TxtRpl { text: "inf", replacment: "\\infty" },
    TxtRpl { text: "arcsin", replacment: "\\arcsin" },
    TxtRpl { text: "arccos", replacment: "\\arccos" },
    TxtRpl { text: "arctan", replacment: "\\arctan" },
    TxtRpl { text: "sinh", replacment: "\\sinh" },
    TxtRpl { text: "cosh", replacment: "\\cosh" },
    TxtRpl { text: "tanh", replacment: "\\tanh" },
    TxtRpl { text: "coth", replacment: "\\coth" },
    TxtRpl { text: "sin", replacment: "\\sin" },
    TxtRpl { text: "cos", replacment: "\\cos" },
    TxtRpl { text: "tan", replacment: "\\tan" },
    TxtRpl { text: "cot", replacment: "\\cot" },
    TxtRpl { text: "sec", replacment: "\\sec" },
    TxtRpl { text: "csc", replacment: "\\csc" },
    TxtRpl { text: "log", replacment: "\\log" },
    TxtRpl { text: "ln", replacment: "\\ln" },
];

fn do_replacements(text: &str) -> String {
    let mut result = text.to_string();
    for rpl in REPLACMENTS.iter() {
        if !result.starts_with('\\') {
            result = result.replace(rpl.text, rpl.replacment);
        }
    }
    //generate_regex();
    result
}

#[allow(unused)]
pub fn generate_docs() {
    let mut result = "| Token | Replacment | LaTeX |\n|-|-|-|\n".to_string();
    for rpl in REPLACMENTS.iter() {
        result = format!("{}| `{}` | `{}` | ${}$ |\n", result, rpl.text, rpl.replacment, rpl.replacment);
    }
    println!("{}", result);
}

#[allow(unused)]
pub fn generate_regex() {
    let mut result = "(".to_string();
    for rpl in REPLACMENTS.iter() {
        result = format!("{}|{}", result, do_regex_escapes(rpl.text));
    }
    result = format!("{})", result);
    println!("{}", result);
}

static ESCAPES: [char; 2] = [
    '^', '+'
];

fn do_regex_escapes(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        if ESCAPES.contains(&c) {
            result.push('\\');
            result.push('\\');
        }
        result.push(c);
    }
    result
}
 

#[allow(unused)]
impl AstNode for AstText {
    fn evaluate(&self, imported_values: &Vec<TypedValue>) -> Result<TypedValue, String> {
        Err("Can't evaluate text.".to_string())
    }

    fn codegen(&self) -> String {
        do_replacements(&self.value)
    }
}

pub fn parse(tokens: &Vec<AtExpToken>, start: i32) -> Result<OpAstNode, String> {
    if let AtExpToken::Text(value) = &tokens[start as usize] {
        Ok(Some((Rc::new(AstText { value: value.clone() }), 1)))
    } else {
        Ok(None)
    }
}