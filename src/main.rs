use std::fs::File;
use std::io::Read;

struct MyError {
    details: String,
}

impl MyError {
    fn new(msg: &str) -> MyError {
        MyError {
            details: msg.to_string(),
        }
    }
}

impl From<std::io::Error> for MyError {
    fn from(e: std::io::Error) -> Self {
       return MyError::new(&e.to_string());
    }
}

fn main() {
    let username = read_username_from_file();

    match username {
        Ok(username) => println!("Succes! your username is: {}", username),
        Err(e) => panic!("Error occured: {}", e.details),
    };
}

fn read_username_from_file() -> Result<String, MyError> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
