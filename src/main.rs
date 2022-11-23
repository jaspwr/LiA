use std::env;
use crate::{utils::*, hierarchy::Node};

mod codegen;
mod utils;
mod parser_modules;
mod tokeniser;
mod hierarchy;
mod hierachy_construction;


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
    let doc = hierachy_construction::contruct_doc(tokens);
    println!("{}", doc.codegen());
}