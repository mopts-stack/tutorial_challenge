struct Uppercase(String);

impl From<String> for Uppercase {
    fn from(data: String) -> Self {
        Uppercase(data.to_uppercase())
    }
}

impl From<&str> for Uppercase {
    fn from(data: &str) -> Self {
        Uppercase(data.to_uppercase())
    }
}

fn example1() {
    let upper = Uppercase::from("lowercase");
    println!("{}", upper.0);
    let upper: Uppercase = "lowercase".into();
    println!("{}", upper.0);
}

enum KeyPress {
    Down,
    Up,
}

struct KeyEvent {
    keycode: u16,
    state: KeyPress,
}

enum InputEvent {
    Key(u16, KeyPress),
    Mouse,
}

impl From<KeyEvent> for InputEvent {
    fn from(ev: KeyEvent) -> Self {
        InputEvent::Key(ev.keycode, ev.state)
    }
}

fn example2() {
    let key_ev = KeyEvent {
        keycode: 5,
        state: KeyPress::Down,
    };

    let input_ev = InputEvent::from(key_ev);

    let key_ev = KeyEvent {
        keycode: 5,
        state: KeyPress::Down,
    };
    let input_ev: InputEvent = key_ev.into();

    println!("conversion is done");
}

use thiserror::Error;

#[derive(Debug, Error)]
enum NetworkError {
    #[error("connection timed out")]
    Timeout,
}

#[derive(Debug, Error)]
enum DatabaseError {
    #[error("error querying database")]
    QueryFailure,
}

#[derive(Debug, Error)]
enum ApiError {
    #[error("network error: {0}")]
    Network(#[from] NetworkError),
    #[error("database error: {0}")]
    Database(#[from] DatabaseError),
}

// The following commented codes can be done via `thiserror` crate's macro #[from] in the ApiError above

// impl From<NetworkError> for ApiError {
//     fn from(err: NetworkError) -> Self {
//         Self::Network(err)
//     }
// }

// impl From<DatabaseError> for ApiError {
//     fn from(err: DatabaseError) -> Self {
//         Self::Database(err)
//     }
// }

fn do_stuff() -> Result<(), ApiError> {
    Err(NetworkError::Timeout)?
}

fn example3() {
    match do_stuff() {
        Ok(_) => println!("it works!"),
        Err(e) => println!("error: {}", e),
    }
}

pub fn start() {
    example1();
    example2();
    example3();
}
