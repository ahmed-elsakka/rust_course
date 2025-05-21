use std;
fn main() {
    let mut name = String::new();

    println!("Enter your name: ");

    std::io::stdin().read_line(&mut name).expect("Failed to read from the command line");

    name = name.trim().to_string();

    println!("Hello {}!", name);
}
