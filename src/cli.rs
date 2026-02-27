use std::rc::Rc;

use crate::compiler::Job;
use owo_colors::OwoColorize;

#[derive(Clone)]
enum Flag {
    TakesNextArg(Rc<dyn Fn(&mut Job, String) -> ShouldContinue>),
    OnlySelf(Rc<dyn Fn(&mut Job) -> ShouldContinue>),
}

pub fn print_info(msg: String) {
    println!("[{}] {}", "INFO".yellow(), msg);
}

fn parse_flag(flag: &str) -> Result<Flag, String> {
    // TODO: Compound small flags. Not needed for now.
    match flag {
        "-o" => Ok(Flag::TakesNextArg(Rc::new(
            |job: &mut Job, arg: String| -> ShouldContinue {
                job.output_path = arg;
                ShouldContinue::Continues
            },
        ))),
        "--pdflatex" => Ok(Flag::OnlySelf(Rc::new(|job: &mut Job| -> ShouldContinue {
            job.pdflatex = true;
            ShouldContinue::Continues
        }))),
        "--html" => Ok(Flag::OnlySelf(Rc::new(|job: &mut Job| -> ShouldContinue {
            job.html = true;
            ShouldContinue::Continues
        }))),
        "--watch" | "-w" => Ok(Flag::OnlySelf(Rc::new(|job: &mut Job| -> ShouldContinue {
            job.watches = true;
            println!("Watching \"{}\" for new changes...", job.input_path.clone());
            ShouldContinue::Continues
        }))),
        "--debug-printing" => Ok(Flag::OnlySelf(Rc::new(|job: &mut Job| -> ShouldContinue {
            job.debug_printing = true;
            ShouldContinue::Continues
        }))),
        "--chain" | "-c" => Ok(Flag::TakesNextArg(Rc::new(
            |job: &mut Job, arg: String| -> ShouldContinue {
                job.chained_command = Some(arg);
                ShouldContinue::Continues
            },
        ))),
        "--help" | "-h" => Ok(Flag::OnlySelf(Rc::new(
            |_job: &mut Job| -> ShouldContinue {
                println!("[?] LiA Compiler {} Help", env!("CARGO_PKG_VERSION"));
                println!("--------------------------------");
                println!(
                    "For language documentation, visit {}",
                    "https://github.com/jaspwr/LiA/blob/main/docs.md".blue()
                );
                println!("Usage: lia [flags] [input files]");
                println!("Flags:");
                println!("  -o [output file] - Sets the output file.");
                println!("  -w / --watch - Watch file for changes and automatically recompile.");
                println!("  --pdflatex - Run pdflatex on the output file after compilation.");
                println!("  -c / --chain [command] - Chain a command to run after compilation.");
                println!("  --help - Prints this help message.");
                println!("  --version - Prints the version of the LiA.");
                ShouldContinue::Aborts
            },
        ))),
        "--version" | "-v" => Ok(Flag::OnlySelf(Rc::new(
            |_job: &mut Job| -> ShouldContinue {
                println!("LiA Compiler Version {}", env!("CARGO_PKG_VERSION"));
                ShouldContinue::Aborts
            },
        ))),
        _ => Err(format!("Unrecognised flag: {flag}")),
    }
}

enum ShouldContinue {
    Continues,
    Aborts,
}

pub fn parse_args(args: Vec<String>) -> Result<Vec<Job>, String> {
    // This returns a vec beacuase eventually it may be possible to compile multiple files at once.
    let mut ret: Vec<Job> = Vec::new();
    let mut working_job = Job::default();
    let mut flag: Option<Flag> = None;
    let mut first = true;
    let mut file_count = 0;

    for arg in args {
        if first {
            first = false;
            continue;
        }
        if arg[0..1] == *"-" {
            if flag.is_some() {
                return Err(format! {"Expected value after flag; got {arg}."});
            }
            let fl = parse_flag(&arg)?;
            flag = Some(fl.clone());
            if let Flag::OnlySelf(f) = fl {
                match f(&mut working_job) {
                    ShouldContinue::Continues => {}
                    ShouldContinue::Aborts => {
                        return Ok(vec![]);
                    }
                };
                flag = None;
            }
        } else {
            match flag.clone() {
                Some(fl) => match fl {
                    Flag::TakesNextArg(f) => {
                        match f(&mut working_job, arg) {
                            ShouldContinue::Continues => {}
                            ShouldContinue::Aborts => {
                                return Ok(vec![]);
                            }
                        };
                        flag = None;
                    }
                    _ => {
                        panic!("Should never be here.")
                    }
                },
                None => {
                    // Flagless arg
                    working_job.input_path = arg.clone();
                    file_count += 1;
                    // Remove if adding multiple file support
                    if file_count > 1 {
                        return Err(format! {"Unexpected argument \"{arg}\"."});
                    }
                }
            }
        }
    }
    if flag.is_some() {
        return Err("Expected value after last flag. Aborted.".to_string());
    }
    if file_count == 0 {
        return Err("No file was provided. Aborted.".to_string());
    }
    // Default output path if not specified
    if working_job.output_path.is_empty() {
        let input = working_job.input_path.clone();
        if input.len() > 4 && input[input.len() - 4..] == *".lia" {
            working_job.output_path = input[0..input.len() - 4].to_string() + ".tex";
        } else {
            working_job.output_path = input + ".tex";
        }
    }
    ret.push(working_job);
    Ok(ret)
}
