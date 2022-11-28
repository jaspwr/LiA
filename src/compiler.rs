use crate::hierarchy::Node;
use crate::tokeniser;
use crate::utils::{write_utf8_file, load_utf8_file};
use crate::hierachy_construction;

#[derive(Default, Clone)]
pub struct Job {
    pub input_path: String,
    pub output_path: String,
    pub watches: bool,
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