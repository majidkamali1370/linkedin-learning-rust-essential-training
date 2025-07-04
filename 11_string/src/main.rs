fn main() {
    let mut my_string = String::from("Hello"); // Create String variable from slice
    println!("{my_string}");
    my_string.push_str(" World!"); // Append string slice
    println!("{my_string}");
}
