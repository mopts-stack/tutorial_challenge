mod async_await;
mod avoid_allocations;
mod builder_pattern;
mod function_arguments;
mod generics_example;
mod iterating_options;
mod json_results;
mod lifetime_demo;
mod map_combinators;
mod user_input;
mod various_challanges;

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

    // json result
    match json_results::start().await {
        Ok(_) => println!("Success"),
        Err(e) => println!("error: {:#}", e),
    }

    // various challanges
    various_challanges::start();

    // map combinators
    map_combinators::start();

    user_input::start();

    generics_example::start();

    lifetime_demo::start();
}
