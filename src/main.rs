#[macro_use]
extern crate derive_builder;

mod avoid_allocations;
mod builder_pattern;
mod function_arguments;
mod iterating_options;

fn main() {
    println!("Hello, world!");

    avoid_allocations::start();
    iterating_options::start();
    builder_pattern::start();
    function_arguments::start();
}
