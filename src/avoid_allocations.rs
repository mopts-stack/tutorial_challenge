use std::mem;

#[derive(Debug)]
enum User {
    Reader { name: String },
    Writer { name: String },
    Admin { name: String },
}

fn promote(u: &mut User) {
    use User::*;

    *u = match u {
        /* // wrong way that encounters heap allocations
        Reader { name } => Writer { name: name.clone() }, // extra heap allocation is happening here
        Writer { name } => Admin { name: name.clone() },  // extra heap allocation is happening here
        */
        Reader { name } => Writer {
            name: mem::take(name),
        }, // no allocation
        Writer { name } => Admin {
            name: mem::take(name),
        }, // no allocation
        Admin { name: _ } => return,
    }
}

pub fn start() {
    let mut user = User::Reader {
        name: "Navid".to_string(),
    };
    println!("{user:?}");

    promote(&mut user);
    println!("{user:?}");

    promote(&mut user);
    println!("{user:?}");
}
