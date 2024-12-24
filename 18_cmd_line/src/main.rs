use std::env;

fn main() {
    for (index, value) in env::args().enumerate() {
        println!("The argument {} is {}", index, value);
    }
}
