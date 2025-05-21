use std::collections::HashMap;
fn main() {
    let mut scores: HashMap<&str, i32> = HashMap::new();

    //scores.insert("Alice", 70);
    scores.insert("John", 50);


    let alice_score = scores.get("Alice");
    match alice_score {
        Some(value) => {
            println!("{}", value);
        } ,
        None => println!("Entry doesn't exist")
    }

    let vec = vec![1];
    let value = vec.get(2);

    match value {
        Some(v) => println!("{}", v),
        None => println!("Index is not valid")
    }
    
    scores.entry("Alice").or_insert(80);

    //scores.remove("John");

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    println!("{:#?}", scores);
}
