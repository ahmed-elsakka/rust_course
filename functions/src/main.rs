fn add(a: i32, b: i32) -> i32 {
    //return a + b;
    let result = a + b;
    //return result;
    result
}
fn main() {
    let result = add(1, 5);

    if result > 5 {
        println!("{} is larger than 5.", result);
    } else if result == 5 {
        println!("{} equals to 5.", result);
    } else {
        println!("{} is less than 5.", result);
    }
}
