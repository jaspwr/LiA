use std::env;
use crate::utils::*;

mod codegen;
mod utils;
mod lang_modules;
mod tokeniser;


fn main() {
    let args: Vec<String> = env::args().collect();
    // TODO: parse args properly
    let temp_path = args.last();
    if temp_path.is_none() || args.len() < 2 {
        println!("Not enough arguments were provided. Aborted");
        return;
    }
    let temp_path = temp_path.unwrap();
    let lia_file = match load_utf8_file(temp_path.clone()) {
        Ok(contents) => { contents },
        Err(e) => { println!("FILE ERROR: {}. Aborted.",e); return; },
    };
    let tokens = tokeniser::to_tokens(lia_file);
    println!("{:?}", tokens);
}