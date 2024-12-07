fn main() {
    let mut s = String::from("Hello");

    borrow_string(&s);
    println!("s = {s}");

    borrow_and_modify_string(&mut s);
    println!("s = {s}");
}

fn borrow_string(s: &String) {
    println!("s.len = {}", s.len());
}

fn borrow_and_modify_string(s: &mut String) {
    s.push_str(", this is a suffix");
}
