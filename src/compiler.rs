use std::process::Command;

use crate::hierarchy::Node;
use crate::tokeniser;
use crate::utils::{write_utf8_file, load_utf8_file};
use crate::hierachy_construction;

#[derive(Default, Clone)]
pub struct Job {
    pub input_path: String,
    pub output_path: String,
    pub chained_command: Option<String>,
    pub watches: bool,
    pub debug_printing: bool
}

pub fn compile (job: Job) -> Result<(), String> {
    let lia_file = match load_utf8_file(job.input_path) {
        Ok(contents) => { contents },
        Err(e) => { return Err(format!("{}. Aborted.",e)); }
    };
    let output = run_compiler(lia_file)?;
    if job.debug_printing {
        println!("{}", output);
    }
    let file_res = write_utf8_file(job.output_path, output);

    if job.chained_command.is_some() {
        let command = job.chained_command.unwrap();        
        if cfg!(target_os = "windows") {
            if Command::new("cmd")
                .args(&["/C", &command])
                .spawn()
                .is_err() {
                    return Err(format!("Failed to run command \"{}\".", command));
                }
        } else {
            if Command::new("sh")
                .args(&["/C", &command])
                .spawn()
                .is_err() {
                    return Err(format!("Failed to run command \"{}\".", command));
                }
        };
    }

    match file_res {
        Ok(_) => { Ok(()) },
        Err(e) => { Err(format!("{}. Aborted.",e)) }
    }
}

pub fn run_compiler(lia_file: String) -> Result<String, String> {
    let tokens = tokeniser::to_tokens(lia_file);
    let doc = hierachy_construction::contruct_doc(tokens)?;
    let output = doc.codegen();
    Ok(output)
}