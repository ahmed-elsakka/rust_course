fn main() {
    let file_open_result =
     std::fs::File::open("hellos.txt");

    /*match file_open_result {
        Ok(file) => println!("File opened!"),
        Err(err) => println!("Error: {}", err)
    }*/

    if let Ok(_) =  file_open_result {
        println!("File opened!");
    } else if let Err(e) = file_open_result {
        println!("Error: {}", e);
    }

}
