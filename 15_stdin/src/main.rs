use std::io;

fn main() {
    let mut name = String::new();
    println!("Enter your name: ");
    let read_result = io::stdin().read_line(&mut name);
    println!("Hello {name}!");
    println!("Read {} characters from stdin", read_result.unwrap());
}
