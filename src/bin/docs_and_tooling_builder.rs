use std::error::Error;

use lia::{*, utils::{load_utf8_file, write_utf8_file}};

static DOCS_FILES: [&str; 2] = ["README.md", "docs.md"];

fn main() {
    match build_docs() {
        Ok(_) => println!("Documentation built successfully!"),
        Err(e) => println!("Error building documentation: {}", e),
    }
}

fn build_docs() -> Result<(), Box<dyn Error>> {
    for path in DOCS_FILES {
        let f = load_utf8_file(path.to_string())?;
        let f = f.replace("{{VERSION}}", env!("CARGO_PKG_VERSION"));
        write_utf8_file(path.to_string(), f)?;
    }
    Ok(())
}