fn main() {
    let mut tup: (i32, f64, char) = (42, 6.9, 'x');

    let first_element = tup.0;
    let second_element = tup.1;
    let third_element = tup.2;

    tup.0 = 70;

    let (a, b, c) = tup;
    println!("{}, {}, {}", a, b , c);
}
