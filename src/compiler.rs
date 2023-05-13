use std::path::PathBuf;
use std::process::Command;
use std::thread;
use std::time::Duration;

use crate::hierarchy::Node;
use crate::hierarchy_construction;
use crate::tokeniser;
use crate::utils::{load_utf8_file, write_utf8_file};

#[derive(Default, Clone)]
pub struct Job {
    pub input_path: String,
    pub output_path: String,
    pub chained_command: Option<String>,
    pub watches: bool,
    pub debug_printing: bool,
    pub pdflatex: bool,
}

pub fn compile(job: Job) -> Result<(), String> {
    let lia_file = match load_utf8_file(&job.input_path) {
        Ok(contents) => contents,
        Err(e) => {
            return Err(format!("{}. Aborted.", e));
        }
    };
    let output = run_compiler(lia_file, job.clone())?;
    if job.debug_printing {
        println!("{}", output);
    }

    let output_path = job.output_path.clone();

    let file_res = write_utf8_file(job.output_path, output);

    if let Some(command) = job.chained_command {
        let mut spl = command.split(" ");
        let cmd: &str = spl.next().unwrap_or("");
        let args = spl.filter(|s| s.len() > 0).collect::<Vec<&str>>();

        let mut child = Command::new(cmd).args(&args).spawn();
        if !wait_for_child(&mut child) {
            return Err(format!("Failed to run command \"{}\".", command));
        }
    }

    if job.pdflatex {
        let abs_path = PathBuf::from(output_path).canonicalize().unwrap(); //
        let mut child = Command::new("pdflatex")
            .arg(abs_path)
            .spawn();

        if !wait_for_child(&mut child) {
            return Err("Failed to run pdflatex. Is it installed?".to_string());
        }
    }

    match file_res {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{}. Aborted.", e)),
    }
}

fn wait_for_child(child: &mut Result<std::process::Child, std::io::Error>) -> bool {
    if let Ok(child) = child {
        if let Ok(status) = child.wait() {
            return status.success();
        }
    }
    false
}

pub fn run_compiler(lia_file: String, job: Job) -> Result<String, String> {
    let tokens = tokeniser::to_tokens(lia_file);
    let doc = hierarchy_construction::contruct_doc(tokens, job)?;
    let output = doc.codegen();
    Ok(output)
}
