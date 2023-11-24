use std::collections::HashMap;

#[derive(Debug)]
struct User {
    user_id: i32,
    name: String,
}

/// Locates a user id based on the name
fn find_user(name: &str) -> Option<i32> {
    let name = name.to_lowercase();
    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(5),
        "katie" => Some(9),
        _ => None,
    }
}

pub fn start() {
    let users = ["navid", "sam", "james", "katie"];
    for user_name in users {
        let user = find_user(user_name).map(|u_id| User {
            user_id: u_id,
            name: user_name.to_owned(),
        });
        match user {
            Some(user) => println!("{:#?}", user),
            None => println!("user '{}' not found", user_name),
        }
    }
}
