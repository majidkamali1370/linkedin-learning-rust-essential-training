fn main() {
    let n = 10;
    let s = "This is a string";

    say_hello();
    say_a_number(n);
    say_a_string(s);
    say_a_number(return_subtract_as_i32(100, 125));
}

// Unlike C/C++, functions can be defined after main function
fn say_hello() {
    println!("Hello :)");
}

// Pass a 32-bit integer by value
fn say_a_number(number: i32) {
    println!("Number is {}", number);
}

// Pass a string slice by reference (borrowed)
fn say_a_string(s: &str) {
    println!("String is '{}'", s);
}

// Return type is shown by an arrow at the end
fn return_subtract_as_i32(a: u8, b: u8) -> i32 {
    return a as i32 - b as i32;
}
