use std::time::Duration;
use tokio::time::sleep;

pub async fn start(i: i32) {
    println!("[#{i}] async result");
    my_function().await;
}

async fn my_function() {
    println!("I'm an async function!");
    let s1 = read_from_database().await; // .await to async will attempt the function to completion otherwise a Future will be returned
    println!("First result: {s1}");
    let s2 = read_from_database().await;
    println!("Second result: {s2}");
}

// the syntax above is a syntactic sugar for the following code
/*
// a simple iplementation
trait Future {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending,
}

fn my_function() -> impl Future<Output = ()> {
    println!("I'm an async function!");
}
*/

// we don not want heavy cpu intensive operations in async
async fn read_from_database() -> String {
    sleep(Duration::from_millis(50)).await; // just current async operation
    "DB Result".to_owned()
}
