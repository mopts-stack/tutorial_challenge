// Notes:
// * See the stdlib docs for the "chunks" method on "slice" for more info

fn data() -> &'static [u64] {
    &[5, 5, 4, 4, 3, 3, 1]
}

fn process_chunk(data: &[u64]) {
    match data {
        [lhs, rhs] => println!("{}+{}={}", lhs, rhs, lhs + rhs),
        [single] => println!("Unpaired value: {}", single),
        [] => println!("data stream complete"),
        [..] => unreachable!("chunk size should be at most 2"),
    }
}

pub fn start() {
    // 'stream' is an iterator of Option<&[u64]>
    let stream = data().chunks(2); // slices of 2 member size slices

    for chunk in stream {
        process_chunk(chunk);
    }
}
