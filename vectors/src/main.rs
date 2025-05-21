fn main() {
    // 1. Creating vectors
    let mut numbers: Vec<i32> = Vec::new(); // empty vector
    let mut primes = vec![2, 3, 5, 7, 11]; // initialized vector

    // 2. Adding elements
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);

    // 3. Accessing elements by indexing (unsafe if out of bounds)
    println!("numbers[1] = {}", numbers[1]); // 20

    // 5. Iterating over vectors (immutable)
    println!("Primes:");
    for p in &primes {
        println!("{}", p);
    }

    // 7. Removing elements
    let last = numbers.pop(); // removes last element
    println!("Popped last number: {:?}", last);
    
    if !primes.is_empty() {
        let removed = primes.remove(0); // removes first element
        println!("Removed first prime: {}", removed);
    }


    // 9. Sorting and reversing
    primes.push(13);
    primes.push(17);
    primes.sort();
    println!("Sorted primes: {:?}", primes);
    primes.reverse();
    println!("Reversed primes: {:?}", primes);
}
