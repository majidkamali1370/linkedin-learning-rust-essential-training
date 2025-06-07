fn main() {
    let mut people: Vec<String> = Vec::new();

    people.push(String::from("Majid"));
    people.push(String::from("Kamali"));
    people.push(String::from("Another guy"));

    println!("People : {:?}", people);

    let last = people.pop();

    println!("Last item : {:?}", last);

    // let third = &people[3]; // Index out of bound
    let third = people.get(3);

    println!("Third item : {:?}", third);

    let another_vector = vec!["Hello", "World"];

    println!("Initialised vector = {:?}", another_vector);
}
