use std::fs;
use std::io;

fn main() {
    let result = fs::read_to_string("non_existent_file.txt");
    let content = match result {
        Ok(message) => message,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => String::from("File not found."),
            io::ErrorKind::PermissionDenied => {
                String::from("Not enough permission to read the file.")
            }
            _ => String::from("Other kind of error."),
        },
    };

    println!("Content is {:?}", content);
}
