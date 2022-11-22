use crate::codegen::Codegen;
use crate::codegen::Alteration;
use crate::utils::*;

mod codegen;
mod utils;

fn main() {
    let mut codegen = Codegen::default();
    //codegen.append_change(Alteration::Deltetion { position: 0, length: 5 });
    codegen.append_change(Alteration::Insersion { position: 1, value: "🐈".to_string() });
    codegen.append_change(Alteration::Deltetion { position: 1, length: 3 });
    codegen.append_change(Alteration::Insersion { position: 1, value: "🐈".to_string() });
    codegen.append_change(Alteration::Deltetion { position: 1, length: 3 });
    codegen.append_change(Alteration::Insersion { position: 0, value: "🐈".to_string() });
    let mut meow_id = byte_by_byte_token_identifier(String::from("meo🐈w"));
    let a = String::from(" meo🐈w; fs     \\sex,    owo  meo🐈w;  ");
    let fuck = a.as_bytes();
    println!("{}", extract_next_token(&fuck, 13).unwrap().0);
    println!("{}", extract_next_token(&fuck, 0).unwrap().0);
    for i in 0..fuck.len() {
        println!("{:?}", meow_id(fuck[i]));
    }

    //codegen.append_change(Alteration::Replacement { position: 2, length: 2, value: "meow".to_string() });
    //println!("{}", codegen.length_difference);
    match codegen.apply(&"inputhjasdhkajsdhjkashdkajsdjkshad".to_string()) {
        Err(e) => println!("CODEGEN ERROR: {}. Aborted.",e),
        Ok(r) => {println!("{:?}", r)}
    }
}
