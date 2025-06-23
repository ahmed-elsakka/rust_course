macro_rules! greet {
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };

    ($name:expr, $country:expr) => {
        println!("Hello, {} from {}!", $name, $country);
    };
}
fn main() {
    greet!("John", "USA");
}
