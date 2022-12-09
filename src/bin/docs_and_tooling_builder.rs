use std::error::Error;

use lia::{*, utils::{load_utf8_file, write_utf8_file}, compiler::run_compiler};

static DOCS_FILES: [&str; 2] = ["README.md", "docs.md"];
//static DOCS_FILES: [&str; 1] = ["README.md"];
static VER_REGEX: &str = "\\b[0-9]+.[0-9]+.[0-9]+\\b";
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
    for path in DOCS_FILES {
        let f = load_utf8_file(path.to_string())?;
        let f = do_compilations(f);
        write_utf8_file(path.to_string(), f)?;
    }
    Ok(())
}

fn do_compilations(s: String) -> String {
    let spl = s.split(COMP_IN_OPEN).collect::<Vec<&str>>();
    let mut out = String::new();
    out.push_str(spl[0]);
    for i in 1..spl.len() {
        let spl_ = spl[i];
        let in_code = strip_codeblock(spl_.split(COMP_IN_CLOSE).collect::<Vec<&str>>()[0].to_string());
        let compilation_result = add_codeblock(run_compiler(in_code).unwrap());
        let a = spl_.split(COMP_OUT_OPEN).collect::<Vec<&str>>();
        out.push_str(COMP_IN_OPEN);
        out.push_str(a[0]);
        out.push_str(COMP_OUT_OPEN);
        let a = a[1].split(COMP_OUT_CLOSE).collect::<Vec<&str>>();
        out.push_str(compilation_result.as_str());
        out.push_str(COMP_OUT_CLOSE);
        out.push_str(a[1]);
    }
    out
}


fn strip_codeblock(s: String) -> String {
    s.split("```tex").collect::<Vec<&str>>()[1].to_string().split("```").collect::<Vec<&str>>()[0].to_string()
}

fn add_codeblock(s: String) -> String {
    format!("\n```tex\n{}\n```\n", s)
}