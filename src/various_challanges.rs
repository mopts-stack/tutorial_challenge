pub fn start() {
    let mut list = vec![1.0, 4.0, 5.0];
    assert_eq!(median(&mut list), Some(4.0));
    let mut list = vec![];
    assert_eq!(median(&mut list), None);
    let mut list = vec![1.0, 4.0, 6.0, 5.0];
    assert_eq!(median(&mut list), Some(4.5));
    println!("median test passed ✅");
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
