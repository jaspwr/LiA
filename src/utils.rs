use std::{fs::File, io::Read};

pub fn load_utf8_file (path: String) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn is_whitespace (char: char) -> bool {
    char == ' ' || char == '\t' || char == '\n' || char == '\r' || char == '\x0C' || char == '\x0B'
}