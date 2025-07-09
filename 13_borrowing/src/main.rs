fn main() {
    let mut s = String::from("Hello");

    borrow_string(&s); // Borrow const string to function
    println!("s = {s}");

    borrow_and_modify_string(&mut s); // Borrow mutable string to function
    println!("s = {s}");
}

// Function signature for const borrowed string
fn borrow_string(s: &String) {
    println!("s.len = {}", s.len());
}

// Function signature for mutable borrowed string
fn borrow_and_modify_string(s: &mut String) {
    s.push_str(", this is a suffix");
}
