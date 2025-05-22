use std::fs::{self, File};
use std::io::{ BufRead, BufReader, BufWriter, Result, Write};
fn main() -> Result<()>{
    /*let file = fs::File::create("large_file.txt")?;
    let mut writer = BufWriter::new(file);

    for i in 1..=10000 {
        writeln!(writer, "This is line number {}", i)?;
    }*/

    let file = File::open("large_file.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }


    Ok(())
}
