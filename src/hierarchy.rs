use std::rc::Rc;

use crate::{utils::indent, hierachy_construction::IndentationType};
pub type NodeList = Vec<Rc<dyn Node>>;
pub type ArgList = Vec<Arg>;

pub struct Doc {
    pub imports: NodeList,
    pub declarations: NodeList,
    pub document: NodeList
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
    fn codegen (&self) -> String;
}

impl Node for Text {
    fn codegen (&self) -> String {
        self.text.clone()
    }
}

impl Node for TexCommand {
    fn codegen (&self) -> String {
        format!("\\{}{}", self.command, (&self.args).into_iter().map(|arg| -> String { 
            arg.codegen() 
        }).collect::<String>())
    }
}

impl Node for TexEnvironment {
    fn codegen (&self) -> String {
        format!("\\begin{{{}}}{}{}\\end{{{}}}", self.name,
        (&self.args).into_iter().map(|arg| -> String { 
            arg.codegen() 
        }).collect::<String>(), 
        indent((&self.children).into_iter().map(|child| -> String { 
            child.codegen() 
        }).collect::<String>(), 1, IndentationType::Space(4)), self.name)
    }
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
            let doc_env = vec![ Rc::new(TexEnvironment {
                name: "document".to_string(),
                args: vec![],
                children: self.document.clone()
            }) as Rc<dyn Node>];
            codegen_nodelist(&doc_env)
        } else {
            "".to_string()
        };
        format!{"{}{}{}", imps, decs, doc}
    }
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

fn codegen_nodelist (list: &NodeList) -> String {
    list.into_iter().map(|node| -> String { 
        node.codegen().clone() 
    }).collect::<String>()
}

impl Arg {
    fn codegen (&self) -> String {
        match &self.arg_type {
            ArgType::Curly => format!{"{{{}}}",codegen_nodelist(&self.arg)},
            ArgType::Square => format!{"[{}]",codegen_nodelist(&self.arg)},
            ArgType::CurlyMultiline => format!{"{{\n{}}}", indent(codegen_nodelist(&self.arg), 1, IndentationType::Space(4))}
        }
    }
}