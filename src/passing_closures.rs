// dyn keyword is required, meaning it might get various values for this specific argument
fn math(a: i32, b: i32, op: Box<dyn Fn(i32, i32) -> i32>) -> i32 {
    op(a, b)
}

pub fn start() {
    let add: Box<_> = Box::new(move |a, b| {
        println!("adding {} to {}", a, b);
        a + b
    });
    let sub = Box::new(move |a, b| {
        println!("subtracting {} from {}", a, b);
        a - b
    });
    let mul = Box::new(move |a, b| {
        println!("multiplying {} by {}", a, b);
        a * b
    });
    println!("{}", math(2, 2, add));
    println!("{}", math(2, 2, sub));
    println!("{}", math(2, 2, mul));
}
