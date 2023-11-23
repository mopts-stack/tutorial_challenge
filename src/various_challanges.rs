use std::fmt::Display;

pub fn start() {
    let mut list = vec![1.0, 4.0, 5.0];
    assert_eq!(median(&mut list), Some(4.0));
    let mut list = vec![];
    assert_eq!(median(&mut list), None);
    let mut list = vec![1.0, 4.0, 6.0, 5.0];
    assert_eq!(median(&mut list), Some(4.5));
    println!("✅ median test(s) passed");

    let list = vec![1, 2, 3, 3, 4, 5, 6, 6, 6, 6];
    assert_eq!(unique(list), [1, 2, 3, 4, 5, 6]);
    let list = vec!['a', 'b', 'a'];
    assert_eq!(unique(list), ['a', 'b']);
    println!("✅ unique test(s) passed");

    let a: &str = "hello";
    let b: String = "Hello".to_string();
    info(&a);
    info(&b);
    println!("✅ print any text type test(s) passed");
}

fn median(v: &mut Vec<f64>) -> Option<f64> {
    if v.is_empty() {
        return None;
    }

    v.sort_by(|x, y| x.partial_cmp(y).unwrap());

    let n_elements: usize = v.len();
    let v_is_even = n_elements & 2 == 0;
    let middle: usize = n_elements / 2;

    let med = if v_is_even {
        // mean or avarage of middle two elements
        (v[middle] + v[middle - 1]) / 2.0
    } else {
        v[middle]
    };

    Some(med)
}

fn unique<T: Ord>(mut v: Vec<T>) -> Vec<T> {
    v.sort(); //_by(|x: &T, y: &T| x.cmp(y));
    v.dedup();
    v
}

fn info<T: Display>(t: &T) {
    println!("{}", t);
}
