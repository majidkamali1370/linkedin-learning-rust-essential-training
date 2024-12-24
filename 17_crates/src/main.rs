use rand::{self, Rng};

fn main() {
    let num1 = rand::random::<u16>();
    let num2 = rand::thread_rng().gen_range(-10..11);
    println!("Random number 1 = {num1}");
    println!("Random number 2 = {num2}");
}
