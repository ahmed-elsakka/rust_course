use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let data = Arc::new(vec![1, 2, 3]);
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..3 {
        let cloned_data = Arc::clone(&data);
        let cloned_counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            //println!("Thread sees: {:?}", cloned_data);
            let mut lock = cloned_counter.lock().unwrap();
            *lock += 1;

            println!("Counter: {}", lock);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
