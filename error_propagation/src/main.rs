use std::fs::File;
use std::io::{self, Read};

fn read_file() -> Result<String, io::Error> {
    let mut file =  File::open("info.txt")?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}

fn main() {
    let content = read_file();
    match content {
        Ok(c) => println!("File content: {}", c),
        Err(e) => println!("Error: {}", e)
    }
}
