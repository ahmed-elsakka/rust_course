fn main() {
    let file_open_result = std::fs::File::open("hello.txt");

    match file_open_result {
        Ok(file) => println!("File opened!"),
        Err(err) => println!("Error: {}", err)
    }
}
