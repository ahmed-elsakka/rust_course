use std::{thread, time::Duration};
use std::sync::mpsc;

fn main() {

    let v = vec![1, 2, 3];

    let (sender, receiver) = mpsc::channel();

    let t = thread::spawn(move || {
        println!("{:?}", v);
        for i in 1..5 {
            println!("From thread: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
        sender.send("Hello from thread t!");
    });

    for i in 1..5 {
        println!("From main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    println!("Waiting for message...");
    let msg = receiver.recv().unwrap();
    println!("{}", msg);
    t.join().unwrap();
}
