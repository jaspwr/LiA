use crate::document::*;
use crate::parse::IndentationType;
use crate::utils::indent;

impl Node for Text {
    fn codegen(&self) -> String {
        self.text.clone().replace("\\@", "@")
    }
}

impl Node for TexCommand {
    fn codegen(&self) -> String {
        format!(
            "\\{}{}",
            self.command,
            self.args
                .iter()
                .map(|arg| -> String { arg.codegen() })
                .collect::<String>()
        )
    }
}

impl Node for TexEnvironment {
    fn codegen(&self) -> String {
        let mut children = self
            .children
            .iter()
            .map(|child| -> String { child.codegen() })
            .collect::<String>();

        if !children.starts_with('\n') {
            children.insert(0, '\n');
        }

        children = children
            .trim_end_matches(' ')
            .trim_end_matches('\t')
            .trim_end_matches('\n')
            .to_string();

        if self.name == "[" {
            format! {"\\[{}\\]",
            indent(children, 1, IndentationType::Space(4))}
        } else {
            format!(
                "\\begin{{{}}}{}{}\\end{{{}}}",
                self.name,
                self.args
                    .iter()
                    .map(|arg| -> String { arg.codegen() })
                    .collect::<String>(),
                indent(children, 1, IndentationType::Space(4)),
                self.name
            )
        }
    }
}

impl Node for Doc {
    fn codegen(&self) -> String {
        let imps = codegen_section(&self.imports);
        let decs = codegen_section(&self.declarations);

        let inner = codegen_nodelist(&self.document)
            .trim_start_matches('\n')
            .trim_end_matches('\n')
            .to_string();

        let spacing = (
            if (!decs.is_empty() && !imps.is_empty())
                || (!inner.is_empty() && !imps.is_empty() && decs.is_empty())
            {
                "\n\n\n"
            } else {
                ""
            },
            if !inner.is_empty() && !decs.is_empty() {
                "\n\n\n"
            } else {
                ""
            },
        );

        let doc = if !inner.is_empty() {
            let inner = indent(inner, 1, IndentationType::Space(4));
            format! {"\\begin{{document}}\n{inner}\\end{{document}}"}
        } else {
            "".to_string()
        };
        format! {"{}{}{}{}{}", imps, spacing.0, decs, spacing.1, doc}
    }
}

fn codegen_section(sec: &NodeList) -> String {
    let decs = if !sec.is_empty() {
        codegen_nodelist(sec).trim_end_matches('\n').to_string()
    } else {
        "".to_string()
    };
    decs
}

fn codegen_nodelist(list: &NodeList) -> String {
    list.iter()
        .map(|node| -> String { node.codegen().clone() })
        .collect::<String>()
}

impl Arg {
    fn codegen(&self) -> String {
        match &self.arg_type {
            ArgType::Curly => format! {"{{{}}}",codegen_nodelist(&self.arg)},
            ArgType::Square => format! {"[{}]",codegen_nodelist(&self.arg)},
            ArgType::CurlyMultiline => {
                format! {"{{\n{}}}", indent(codegen_nodelist(&self.arg), 1, IndentationType::Space(4))}
            }
        }
    }
}
