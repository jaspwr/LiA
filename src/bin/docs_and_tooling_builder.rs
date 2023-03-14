// This is unfinished.

use std::error::Error;

use lia::{
    compiler::run_compiler,
    utils::{load_utf8_file, write_utf8_file},
};

static COMP_IN_OPEN: &str = "[COMPILATION_INPUT_START]: <> (Do not remove this line.)";
static COMP_IN_CLOSE: &str = "[COMPILATION_INPUT_END]: <> (Do not remove this line.)";
static COMP_OUT_OPEN: &str = "[COMPILATION_RESULT_START]: <> (Do not remove this line.)";
static COMP_OUT_CLOSE: &str = "[COMPILATION_RESULT_END]: <> (Do not remove this line.)";

fn main() {
    match build_docs() {
        Ok(_) => println!("Documentation built successfully!"),
        Err(e) => println!("Error building documentation: {}", e),
    }
}

fn build_docs() -> Result<(), Box<dyn Error>> {
    proc_file("docs.md", true)?;
    proc_file("README.md", false)?;
    Ok(())
}

fn proc_file(path: &str, strips_doc_env: bool) -> Result<(), Box<dyn Error>> {
    let f = load_utf8_file(&path.to_string())?;
    let f = do_compilations(f, strips_doc_env);
    write_utf8_file(path.to_string(), f)?;
    Ok(())
}

// Imperative mess...
fn do_compilations(s: String, strips_doc_env: bool) -> String {
    let spl = s.split(COMP_IN_OPEN).collect::<Vec<&str>>();
    let mut out = String::new();
    out.push_str(spl[0]);
    for i in 1..spl.len() {
        let spl_ = spl[i];
        let in_code =
            strip_codeblock(spl_.split(COMP_IN_CLOSE).collect::<Vec<&str>>()[0].to_string());
        let compilation_result = run_compiler(in_code).unwrap();
        let a = spl_.split(COMP_OUT_OPEN).collect::<Vec<&str>>();
        out.push_str(COMP_IN_OPEN);
        out.push_str(a[0]);
        out.push_str(COMP_OUT_OPEN);
        let a = a[1].split(COMP_OUT_CLOSE).collect::<Vec<&str>>();
        let code_out = if strips_doc_env {
            strip_doc_env(compilation_result.to_string())
        } else {
            compilation_result.to_string()
        };
        out.push_str(&add_codeblock(code_out));
        out.push_str(COMP_OUT_CLOSE);
        out.push_str(a[1]);
    }
    out
}

fn strip_codeblock(s: String) -> String {
    s.split("```tex").collect::<Vec<&str>>()[1]
        .to_string()
        .split("```")
        .collect::<Vec<&str>>()[0]
        .to_string()
}

fn add_codeblock(s: String) -> String {
    format!("\n```tex\n{}\n```\n", s)
}

fn strip_doc_env(s: String) -> String {
    let a = s.split("\\begin{document}\n").collect::<Vec<&str>>();
    if a.len() == 1 {
        return a[0].to_string();
    }
    let a = a[1].split("\\end{document}").collect::<Vec<&str>>();
    unindent(a[0].to_string())
}

fn unindent(s: String) -> String {
    let mut out = String::new();
    for line in s.lines() {
        out.push_str(&line[4..].to_string());
        out.push_str("\n");
    }
    out.pop();
    out
}
