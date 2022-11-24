use crate::{utils::{load_utf8_file, write_utf8_file}, hierachy_construction, tokeniser, hierarchy::Node};

#[derive(Default, Clone)]
pub struct Job {
    pub input_path: String,
    pub output_path: String,
    pub debug_printing: bool
}

pub fn compile (job: Job) -> Result<(), String> {
    let lia_file = match load_utf8_file(job.input_path) {
        Ok(contents) => { contents },
        Err(e) => { return Err(format!("{}. Aborted.",e)); }
    };
    let tokens = tokeniser::to_tokens(lia_file);
    let doc = hierachy_construction::contruct_doc(tokens)?;
    let output = doc.codegen();
    if job.debug_printing {
        println!("{}", output);
    }
    match write_utf8_file(job.output_path, output) {
        Ok(_) => { Ok(()) },
        Err(e) => { Err(format!("{}. Aborted.",e)) }
    }
}