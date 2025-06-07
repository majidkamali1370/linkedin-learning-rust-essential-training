use std::collections::HashMap;

fn main() {
    let mut people: HashMap<i32, String> = HashMap::new();

    people.insert(1001, String::from("Majid"));
    people.insert(1002, String::from("Kamali"));
    people.insert(1002, String::from("Kamali 2")); // Update map
    people.entry(1003).or_insert(String::from("Another guy")); // Get entry or insert if not exists

    let another_guy = people.entry(1003).or_default(); // Insert default value if entry does not exist, then return entry

    (*another_guy).push_str(" 2"); // Update returned entry

    println!("People = {:?}", people);
}
