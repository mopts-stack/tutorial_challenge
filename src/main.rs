#[macro_use]
extern crate derive_builder;

mod async_await;
mod avoid_allocations;
mod builder_pattern;
mod function_arguments;
mod iterating_options;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    avoid_allocations::start();
    iterating_options::start();
    builder_pattern::start();
    function_arguments::start();

    // important to use any async runtime like tokio and mark main as an async as well
    let result = async_await::start(10);
    println!("Here is some text after callling an async function");
    result.await;

    let mut handles = vec![];
    for i in 0..4 {
        let handle = tokio::spawn(async move {
            async_await::start(i).await;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
