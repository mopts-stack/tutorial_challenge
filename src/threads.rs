use std::thread::{self, JoinHandle};
use std::time::Duration;

pub fn start() {
    let iterations = 10;
    let a = thread::spawn(move || {
        for i in 1..=iterations {
            println!("---A:{}", i);
        }
    });
    let b = thread::spawn(move || {
        for i in 1..=iterations {
            println!("===B:{}", i);
        }
    });

    a.join();
    b.join();

    let value: JoinHandle<usize> = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        42
    });

    println!("Waiting on thread");

    match value.join() {
        Ok(n) => println!("value: {}", n),
        Err(e) => println!("error joining thread: {:?}", e),
    }

    let data = vec!['a', 'b', 'c'];
    let caps = thread::spawn(move || {
        let data: Vec<char> = data.iter().map(|c| c.to_ascii_uppercase()).collect();
        data
    });

    println!("waiting for value...");

    match caps.join() {
        Ok(n) => println!("value: {:?}", n),
        Err(e) => println!("error joining thread: {:?}", e),
    }
}
