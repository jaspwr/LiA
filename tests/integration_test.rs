
use std::fs::remove_file;

use lia::compiler::*;
use lia::utils::*;
use text_diff::print_diff;

#[test]
fn general() {
    test_compilation_result(
    "tests/general.lia".to_string(),
    "tests/temp_general.tex".to_string(),
    "tests/general_out.tex".to_string());
}

#[test]
fn regular_tex() {
    test_compilation_result(
    "tests/regular_tex.lia".to_string(),
    "tests/temp_regular_tex.tex".to_string(),
    "tests/regular_tex_out.tex".to_string());
}

#[test]
fn functions() {
    test_compilation_result(
    "tests/functions.lia".to_string(),
    "tests/temp_functions.tex".to_string(),
    "tests/functions_out.tex".to_string());
}

#[test]
fn readme_example() {
    test_compilation_result(
    "tests/readme_example.lia".to_string(),
    "tests/temp_readme_example.tex".to_string(),
    "tests/readme_example_out.tex".to_string());
}

#[test]
fn equations() {
    test_compilation_result(
    "tests/equations.lia".to_string(),
    "tests/temp_equations.tex".to_string(),
    "tests/equations_out.tex".to_string());
}

fn test_compilation_result(input_path: String, output_path: String, correct_output_path: String) -> () {
    let job = Job {
        input_path: input_path.clone(),
        output_path: output_path.clone(),
        watches: false,
        debug_printing: false,
        chained_command: None
    };
    match compile(job.clone()) {
        Ok(_) => { 
            let output = load_utf8_file(job.output_path.clone()).unwrap();
            let correct_output = load_utf8_file(correct_output_path.clone()).unwrap();
            remove_file(job.output_path.clone()).unwrap();
            if output != correct_output {
                print_diff(&output, &correct_output, " ");
                panic!("Output is not correct.");
            } else {
                ()
            }
        },
        Err(e) => { panic!("{}",format!{"Compiler Error: {}", e}) }
    };
}