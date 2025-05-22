use std::fs;
use std::io;

fn main() -> io::Result<()> {
    //fs::create_dir("new_folder")?;
    //fs::remove_dir("new_folder")?;
    //fs::remove_file("file.txt")?;
    //fs::copy("file.txt", "src/file_new.txt")?;
    //fs::rename("file_new.txt", "file_renamed.txt")?;
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        println!("{:?}", entry);
    }
    Ok(())
}
