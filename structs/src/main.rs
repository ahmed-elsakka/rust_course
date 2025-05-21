trait Greet {
    fn say_hello(&self);
}

#[derive(Debug)]
struct Person {
    name: String
}

impl Greet for Person {
    fn say_hello(&self) {
        println!("Hello, my name is {}", self.name);
    }
}
fn main() {
    let person = Person {name: String::from("Alice")};
    person.say_hello();

    println!("{:#?}", person);
}
