use std::fmt::{Display, Formatter, Result};

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

    let mut users: Vec<&str> = vec!["Todd", "amy", "mike99", "Jennifer", "alison"];
    println!("unsorted:                  {:?}", &users);
    sort_usernames(&mut users);
    println!("sorted case insensitively: {:?}", &users);
    let mut users: Vec<&str> = vec!["Todd", "amy", "mike99", "Jennifer", "alison"];
    users.sort();
    println!("sorted using .sort():      {:?}", &users);
    println!("✅ case insensitive sort test(s) passed");

    let greeting = "Hello, World";
    println!("{}", greeting);
    let greeting_morse = greeting.to_string().to_morse_code();
    print_morse_code(&greeting_morse);
    println!("✅ convert to morse test(s) passed");

    let nn = vec![];
    assert_eq!(sum_ignore_missing(nn), 0);
    let nn = vec![None, Some(1), Some(5), Some(4), None, None];
    assert_eq!(sum_ignore_missing(nn), 10);
    println!("✅ sum ignore missing test(s) passed");
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

fn sort_usernames<T: AsRef<str>>(usernames: &mut [T]) {
    // simple way
    // usernames.sort_by(|x: &T, y: &T| x.as_ref().to_lowercase().cmp(&y.as_ref().to_lowercase()))

    // more efficient way
    usernames.sort_by_cached_key(|x: &T| x.as_ref().to_lowercase())
}

#[derive(Debug, PartialEq)]
enum Pulse {
    Short,
    Long,
}
type Letter = Vec<Pulse>;
type Message = Vec<Letter>;
trait MorseCode {
    fn to_morse_code(&self) -> Message;
}
impl MorseCode for String {
    fn to_morse_code(&self) -> Message {
        use Pulse::*;

        let mut message = Vec::with_capacity(self.len());
        for c in self.chars() {
            let morse = match c {
                'A' | 'a' => vec![Short, Long],
                'B' | 'b' => vec![Long, Short, Short, Short],
                'C' | 'c' => vec![Long, Short, Long, Short],
                'D' | 'd' => vec![Long, Short, Short],
                'E' | 'e' => vec![Short],
                'F' | 'f' => vec![Short, Short, Long, Short],
                'G' | 'g' => vec![Long, Long, Short],
                'H' | 'h' => vec![Short, Short, Short, Short],
                'I' | 'i' => vec![Short, Short],
                'J' | 'j' => vec![Short, Long, Long, Long],
                'K' | 'k' => vec![Long, Short, Long],
                'L' | 'l' => vec![Short, Long, Short, Short],
                'M' | 'm' => vec![Long, Long],
                'N' | 'n' => vec![Long, Short],
                'O' | 'o' => vec![Long, Long, Long],
                'P' | 'p' => vec![Short, Long, Long, Short],
                'Q' | 'q' => vec![Long, Long, Short, Long],
                'R' | 'r' => vec![Short, Long, Short],
                'S' | 's' => vec![Short, Short, Short],
                'T' | 't' => vec![Long],
                'U' | 'u' => vec![Short, Short, Long],
                'V' | 'v' => vec![Short, Short, Short, Long],
                'W' | 'w' => vec![Short, Long, Long],
                'X' | 'x' => vec![Long, Short, Short, Long],
                'Y' | 'y' => vec![Long, Short, Long, Long],
                'Z' | 'z' => vec![Long, Long, Short, Short],

                '1' => vec![Short, Long, Long, Long, Long],
                '2' => vec![Short, Short, Long, Long, Long],
                '3' => vec![Short, Short, Short, Long, Long],
                '4' => vec![Short, Short, Short, Short, Long],
                '5' => vec![Short, Short, Short, Short, Short],
                '6' => vec![Long, Short, Short, Short, Short],
                '7' => vec![Long, Long, Short, Short, Short],
                '8' => vec![Long, Long, Long, Short, Short],
                '9' => vec![Long, Long, Long, Long, Short],
                '0' => vec![Long, Long, Long, Long, Long],
                _ => continue,
            };
            message.push(morse);
        }

        message
    }
}
impl Display for Pulse {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Pulse::Short => write!(f, "."),
            Pulse::Long => write!(f, "_"),
        }
    }
}
fn print_morse_code(code: &Message) {
    for letter in code.iter() {
        for pulse in letter.iter() {
            print!("{}", pulse);
        }
        print!(" ");
    }
    println!();
}

fn sum_ignore_missing(numbers: Vec<Option<i32>>) -> i32 {
    numbers.iter().map(|x| x.unwrap_or(0)).sum()
}
