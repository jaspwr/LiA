use std::rc::Rc;
pub type NodeList = Vec<Rc<dyn Node>>;
pub type ArgList = Vec<Arg>;

pub struct Doc {
    pub children: NodeList
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
        (&self.children).into_iter().map(|child| -> String { 
            child.codegen() 
        }).collect::<String>(), self.name)
    }
}

impl Node for Doc {
    fn codegen (&self) -> String {
        codegen_nodelist(&self.children)
    }
}

pub enum ArgType {
    Curly,
    Square,
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
            ArgType::Square => format!{"[{}]",codegen_nodelist(&self.arg)}
        }
    }
}