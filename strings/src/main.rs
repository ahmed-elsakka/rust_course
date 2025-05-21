fn main() {
    let mut string: String = "Hello".to_string();
    //let string_slice: &str = " World";
    let string2: String = " World!".to_string();
    /*string.push_str(string_slice);
    string.push('!');*/

    let concatenated_string = string + &string2;

    println!("{}", concatenated_string);
    println!("{}", concatenated_string.is_empty());
    println!("{}", concatenated_string.contains("World"));
    println!("{}", concatenated_string.to_uppercase());
    println!("{}", concatenated_string.to_lowercase());

    let replaced = concatenated_string.replace("World", "Everyone");
    println!("{}", replaced);
}
