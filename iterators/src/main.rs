fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    for item in v.iter() {
        println!("{}", item);
    }

    let result: Vec<i32> = v
    .into_iter()
    .filter(|x| x % 2 == 0)
    .map(|x| x * 2)
    .collect();

    println!("{:?}", result);

}
