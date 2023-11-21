pub fn start() {
    // Extend a collection
    let grade = Some("A+");
    let grade2 = Some("F+");
    let mut grades = vec!["B-", "C+", "D"];

    println!("{grades:?}");

    if let Some(grade) = grade {
        grades.push(grade);
    }

    println!("{grades:?}");

    // shorter way
    grades.extend(grade2); // option type implements into_iter trait so this works

    println!("{grades:?}");

    // extending an iterator
    // let grades = vec!["B-", "C+", "D"];
    let grades = ["B-", "C+", "D"];
    for grade in grades.iter().chain(grade.iter()).chain(grade2.iter()) {
        println!("{grade}");
    }

    // Filter out none variants
    let grades = vec![Some("A+"), None, Some("B-"), None];
    let grades: Vec<&str> = grades.into_iter().flatten().collect();

    println!("{grades:?}");

    // Map and filter out none variants
    let grades = ["3.8", "B+", "4.0", "A", "2.7"];
    let grades: Vec<f32> = grades.iter().filter_map(|s| s.parse().ok()).collect();

    println!("{grades:?}");
}
