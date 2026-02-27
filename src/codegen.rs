use crate::document::*;
use crate::parse::IndentationType;
use crate::utils::indent;

impl Node for Text {
    fn codegen(&self) -> String {
        self.text.clone().replace("\\@", "@")
    }

    fn codegen_html(&self) -> String {
        self.text.clone()
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

    fn codegen_html(&self) -> String {
        let inner = self
            .args
            .iter()
            .map(|arg| -> String { arg.html_codegen() })
            .collect::<String>();
        match self.command.as_str().trim() {
            "raisebox" => self.args[1].html_codegen(),
            "textbf" => format!("<b>{}</b>\n", inner,),
            "textit" => format!("<i>{}</i>\n", inner,),
            "texttt" => format!("<code>{}</code>\n", inner,),
            "section" => format!("<h1>{}</h1>\n", inner,),
            "subsection" => format!("<h2>{}</h2>\n", inner,),
            "subsubsection" => format!("<h3>{}</h3>\n", inner,),
            "section*" => format!("<h1>{}</h1>\n", inner,),
            "subsection*" => format!("<h2>{}</h2>\n", inner,),
            "subsubsection*" => format!("<h3>{}</h3>\n", inner,),
            "includegraphics" => format!("<img src=\"img/{}.png\">\n", self.args[1].html_codegen()),
            "hline" => format!("<hr>\n"),
            "item" => "</li><li>\n".to_string(),
            "\\" | "linebreak" => "<br>".to_string(),
            "label" => String::new(),
            "Ctrl" => "<span class=\"keystroke\">Ctrl</span>".to_string(),
            "Alt" => "<span class=\"keystroke\">Alt</span>".to_string(),
            "Shift" => "<span class=\"keystroke\">Shift</span>".to_string(),
            _ => format!("<span class=\"{}\">{}</span>", self.command, inner),
        }
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

    fn codegen_html(&self) -> String {
        let children = self
            .children
            .iter()
            .map(|child| -> String { child.codegen_html() })
            .collect::<String>();

        match self.name.as_str() {
            "verbaitum" => format!("<code\n{}</code>\n", children),
            "itemize" => format!("<ul>\n{}</ul>\n", children),
            "enumerate" => format!("<ol>\n{}</ol>\n", children),
            "center" => format!("<center>\n{}</center>\n", children),
            // HACK
            "tabular" => format!(
                "<table><tr><td>\n{}</td></tr></table>\n",
                children
                    .replace("&", "</td><td>")
                    .replace("<br>", "</td></tr><tr><td>")
                    .replace("<hr>", "")
            ),
            _ => format!("<div class=\"{}\">\n{}</div>\n", self.name, children),
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

    fn codegen_html(&self) -> String {
        html_codegen_nodelist(&self.document)
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

fn html_codegen_nodelist(list: &NodeList) -> String {
    list.iter()
        .map(|node| -> String { node.codegen_html().clone() })
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

    fn html_codegen(&self) -> String {
        html_codegen_nodelist(&self.arg)
    }
}
