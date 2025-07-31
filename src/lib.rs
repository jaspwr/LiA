use compiler::*;
use notify::{RecursiveMode, Watcher};
use owo_colors::OwoColorize;
use std::path::Path;
use version::check_for_new_version;

pub mod ast;
pub mod at_expression;
mod bracket_depth;
mod cli;
mod codegen;
pub mod compiler;
mod document;
mod feature_matrix;
pub mod grammar;
mod parse;
mod parser_modules;
mod token;
mod tokenize;
pub mod typed_value;
pub mod utils;
mod version;

pub fn run_from_args(args: Vec<String>) {
    let _ = check_for_new_version();
    let jobs = match cli::parse_args(args) {
        Ok(jobs) => jobs,
        Err(e) => {
            println!("[{}] {}", "Error".red(), e);
            return;
        }
    };
    for job in jobs {
        run_job(&job);

        let name = job.input_path.clone();
        if job.watches {
            let mut pre_hash = utils::hash_file(&job.input_path);
            let mut watcher = notify::recommended_watcher(move |_| {
                let hash = utils::hash_file(&job.input_path);
                if hash == "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855" {
                    // Empty file
                    return;
                }
                if pre_hash != hash {
                    run_job(&job);
                }
                pre_hash = hash;
            })
            .unwrap();
            watcher
                .watch(Path::new(&name), RecursiveMode::Recursive)
                .unwrap();
            loop {
                std::thread::sleep(std::time::Duration::from_millis(1000));
            }
        }
    }
}

fn run_job(job: &Job) {
    match compile(job.clone()) {
        Ok(_) => {
            println!(
                "[{}] Ouput \"{}\".",
                "Success".green(),
                job.output_path.clone()
            );
        }
        Err(e) => {
            println!(
                "[{}] \"{}\" {}",
                "Compiler Error".red(),
                job.input_path.clone(),
                e
            );
        }
    };
}
