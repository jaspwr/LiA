use std::rc::Rc;

pub type NodeList = Vec<Rc<dyn Node>>;
pub type ArgList = Vec<Arg>;

pub struct Doc {
    pub imports: NodeList,
    pub declarations: NodeList,
    pub document: NodeList,
}

#[derive(Debug)]
pub enum DocSection {
    Imports,
    Declarations,
    Document,
}

pub struct Text {
    pub text: String,
}

pub struct TexCommand {
    pub command: String,
    pub args: ArgList,
}

pub struct TexEnvironment {
    pub name: String,
    pub args: ArgList,
    pub children: NodeList,
}

pub trait Node {
    fn codegen(&self) -> String;
}

pub enum ArgType {
    Curly,
    Square,
    CurlyMultiline,
}

pub struct Arg {
    pub arg: NodeList,
    pub arg_type: ArgType,
}
