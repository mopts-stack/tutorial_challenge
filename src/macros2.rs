// Topic: Basic macro repetitions
//
// Requirements:
//   * Create a macro to generate hashmaps.
//   * The macro must be able to accept multiple key/value pairs.
//   * Print out the generated hashmap using the `dbg!` macro to ensure it works.

macro_rules! hashmap {
    (
        $($key:expr => $value:expr),+
        $(,)?
    ) => {{
        let mut hashmap = ::std::collections::HashMap::new();
        $(
            hashmap.insert($key, $value);
        )+
        hashmap
    }};
}

pub fn start() {
    let hashmap = hashmap! {
        1 => "a",
        2 => "b",
        3 => "c",
    };

    println!("{:#?}", hashmap);

    let hashmap = {
        let mut hashmap = std::collections::HashMap::new();
        hashmap.insert(1, "a".to_owned());
        hashmap.insert(2, "b".to_owned());
        hashmap.insert(3, "c".to_owned());
        hashmap
    };
    println!("{:#?}", hashmap);
}
