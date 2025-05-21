fn main() {
    //let arr: [i32; 5] = [0; 5];
    let mut arr = [1, 2, 3, 5];

    println!("Array: {:#?}", arr);

    arr[1] = 5;
    println!("Second element: {}", arr[1]);
    println!("Length of array: {}", arr.len());
}
