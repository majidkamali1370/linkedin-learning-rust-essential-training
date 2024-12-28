use std::mem;

struct Person {
    name: String,
    age: i32,
    weight: f64,
}

fn main() {
    let person = Person {
        name: String::from("Majid"),
        age: 33,
        weight: 75.0,
    };

    println!("person size on stack = {}", mem::size_of_val(&person));

    let boxed_person = Box::new(person);

    println!(
        "boxed person on stack = {}",
        mem::size_of_val(&boxed_person)
    );
    println!(
        "boxed person on heap = {}",
        mem::size_of_val(&*boxed_person)
    );

    let unboxed_person = *boxed_person;

    println!(
        "unboxed person on stack = {}",
        mem::size_of_val(&unboxed_person)
    );
}
