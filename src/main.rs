use std::env;
use compiler::*;
use owo_colors::OwoColorize;

mod compiler;
mod cli;
mod utils;
mod parser_modules;
mod tokeniser;
mod hierarchy;
mod hierachy_construction;


fn main() {
    let args: Vec<String> = env::args().collect();
    
    let jobs = match cli::parse_args(args) {
        Ok(jobs) => { jobs },
        Err(e) => {
            println!("[{}] {}", "Error".red(), e);
            return;
        }
    };
    for job in jobs {
        match compile(job.clone()) {
            Ok(_) => { println!("[{}] Ouput \"{}\".", "Success".green(), job.output_path.clone());},
            Err(e) => { println!("[{}] \"{}\" {}","Compiler Error".red(), job.input_path.clone(), e); }
        };
    }

}