use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;

fn main() -> io::Result<()> {
    // Could use error propagation
    let mut file: fs::File = match fs::File::open("file.txt") {
        Ok(f) => f,
        Err(e) => {
            println!("Error: {}", e);
            return Err(e);
        }
    };

    let mut content = String::new();
    file.read_to_string(&mut content)?;
    content.push('\n');
    println!("File content: {}", content);

    let mut created_file: fs::File = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("created_file.txt")?;
    created_file.write_all(content.as_bytes())?;

    Ok(())
}
