fn main() {
    let n = 10;
    let s = "This is a string";

    say_hello();
    say_a_number(n);
    say_a_string(s);
    say_a_number(return_subtract_as_i32(100, 125));
}

fn say_hello() {
    println!("Hello :)");
}

fn say_a_number(number: i32) {
    println!("Number is {}", number);
}

fn say_a_string(s: &str) {
    println!("String is '{}'", s);
}

fn return_subtract_as_i32(a: u8, b: u8) -> i32 {
    return a as i32 - b as i32;
}
