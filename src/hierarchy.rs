use std::borrow::BorrowMut;
use std::ops::Deref;
use std::rc::Rc;

use crate::hierachy_construction::IndentationType;
use crate::token::Token;
use crate::tokeniser::TokenList;
use crate::utils::indent;
pub type NodeList = Vec<Rc<dyn Node>>;
pub type ArgList = Vec<Arg>;

pub struct Doc {
    pub imports: NodeList,
    pub declarations: NodeList,
    pub document: NodeList
}

pub enum DocSection {
    Imports,
    Declarations,
    Document
}

pub struct Text {
    pub text: String,
}

pub struct TexCommand {
    pub command: String,
    pub args: ArgList
}

pub struct TexEnvironment {
    pub name: String,
    pub args: ArgList,
    pub children: NodeList,
}

pub trait Node {
    fn codegen(&self) -> String;
    fn strip_leading_newlines(&self) -> Option<Rc<dyn Node>>;
    fn strip_tailing_newlines(&self) -> Option<Rc<dyn Node>>;
}

impl Node for Text {
    fn codegen(&self) -> String {
        self.text.clone()
    }

    fn strip_leading_newlines(&self) -> Option<Rc<dyn Node>> {
        let text = self.text.trim_start_matches('\n').to_string();
        Some(Rc::new(Text { text }))
    }

    fn strip_tailing_newlines(&self) -> Option<Rc<dyn Node>> {
        let text = self.text.trim_end_matches('\n').to_string();
        Some(Rc::new(Text { text }))
    }
}

impl Node for TexCommand {
    fn codegen(&self) -> String {
        format!("\\{}{}", self.command, (&self.args).into_iter().map(|arg| -> String { 
            arg.codegen() 
        }).collect::<String>())
    }

    fn strip_leading_newlines(&self) -> Option<Rc<dyn Node>> { None }
    fn strip_tailing_newlines(&self) -> Option<Rc<dyn Node>> { None }
}

impl Node for TexEnvironment {
    fn codegen(&self) -> String {
        let mut children = (&self.children).into_iter().map(|child| -> String { 
            child.codegen() 
        }).collect::<String>();
        if !children.starts_with('\n') {
            children.insert(0, '\n');
        }
        
        // This is a little scuffed.
        children = children.trim_end_matches(' ')
        .trim_end_matches('\t')
        .trim_end_matches('\n')
        .to_string();

        format!("\\begin{{{}}}{}{}\\end{{{}}}", self.name,
        (&self.args).into_iter().map(|arg| -> String { 
            arg.codegen() 
        }).collect::<String>(), 
        indent(children, 1, IndentationType::Space(4)), self.name)
    }

    fn strip_leading_newlines(&self) -> Option<Rc<dyn Node>> { None }
    fn strip_tailing_newlines(&self) -> Option<Rc<dyn Node>> { None }
}

impl Node for Doc {
    fn codegen (&self) -> String {
        let imps = if &self.imports.len() > &0 {
            format!{ "{}\n\n", codegen_nodelist(&self.imports) }
        } else { "".to_string() };
        let decs = if &self.declarations.len() > &0 {
            format!{ "{}\n\n", codegen_nodelist(&self.declarations) }
        } else { "".to_string() };

        let doc = if &self.document.len() > &1 {
            let inner = codegen_nodelist(&self.document)
            .trim_start_matches('\n')
            .trim_end_matches('\n')
            .to_string();
            let inner = indent(inner, 1, IndentationType::Space(4));
            format!{"\\begin{{document}}\n{}\\end{{document}}", inner}
        } else {
            "".to_string()
        };
        format!{"{}{}{}", imps, decs, doc}
    }

    fn strip_leading_newlines(&self) -> Option<Rc<dyn Node>> { None }
    fn strip_tailing_newlines(&self) -> Option<Rc<dyn Node>> { None }
}

pub enum ArgType {
    Curly,
    Square,
    CurlyMultiline,
}

pub struct Arg {
    pub arg: NodeList,
    pub arg_type: ArgType
}

fn codegen_nodelist(list: &NodeList) -> String {
    list.into_iter().map(|node| -> String { 
        node.codegen().clone() 
    }).collect::<String>()
}

impl Arg {
    fn codegen(&self) -> String {
        match &self.arg_type {
            ArgType::Curly => format!{"{{{}}}",codegen_nodelist(&self.arg)},
            ArgType::Square => format!{"[{}]",codegen_nodelist(&self.arg)},
            ArgType::CurlyMultiline => format!{"{{\n{}}}", indent(codegen_nodelist(&self.arg), 1, IndentationType::Space(4))}
        }
    }
}
