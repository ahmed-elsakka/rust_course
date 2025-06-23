fn main() {
    let arr = [1, 2, 3, 4, 5];

    let slice: &[i32] = &arr[1..=3];

    println!("Slice: {:?}", slice);

    let s = String::from("Hello");
    let s_slice: &str = &s[0..2];

    println!("String slice: {:?}", s_slice);
}
