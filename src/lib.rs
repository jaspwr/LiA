use std::path::Path;
use compiler::*;
use owo_colors::OwoColorize;
use notify::{Watcher, RecursiveMode};

pub mod compiler;
pub mod utils;
mod cli;
mod parser_modules;
mod tokeniser;
mod hierarchy;
mod hierachy_construction;

pub fn run_from_args(args: Vec<String>) {
    let jobs = match cli::parse_args(args) {
        Ok(jobs) => { jobs },
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
            let mut watcher = notify::recommended_watcher(move|_| {
                let hash = utils::hash_file(&job.input_path);
                if pre_hash != hash {
                    pre_hash = hash;
                    run_job(&job);
                }
            }).unwrap();
            watcher.watch(Path::new(&name), RecursiveMode::Recursive).unwrap();
            loop { std::thread::sleep(std::time::Duration::from_millis(1000)); }
        }
    }
}

fn run_job(job: &Job) -> () {
    match compile(job.clone()) {
        Ok(_) => { println!("[{}] Ouput \"{}\".", "Success".green(), job.output_path.clone());},
        Err(e) => { println!("[{}] \"{}\" {}","Compiler Error".red(), job.input_path.clone(), e); }
    };
}