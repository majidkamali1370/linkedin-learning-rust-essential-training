use std::io;

fn main() {
    let mut buffer = String::new();
    println!("Enter buffer: ");
    let read_result = io::stdin().read_line(&mut buffer);
    println!("Read {} characters from stdin", read_result.unwrap());

    // let number = buffer.trim().parse::<i32>();
    let number: i32 = buffer.trim().parse().unwrap(); // same as above line

    println!("Entered number is {number}");
}
