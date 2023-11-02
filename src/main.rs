use std::fs::File;
use std::io::{Read, Result};

fn main() {
    let result = read_line_from_file();
    let mut count = 0;
    match result {
        Ok(lines) => {
            for line in lines.trim().split("\n") {
                count += 1;
                println!("{} {}", count, line);
            }
        },
        Err(e) => panic!("Something went wrong {}", e),
    }
}

fn read_line_from_file() -> Result<String> {
    let greeting_file_result = File::open("greeting.txt");

    let mut greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut line = String::new();

    match greeting_file.read_to_string(&mut line) {
        Ok(_) => Ok(line),
        Err(e) => Err(e),
    }
}
