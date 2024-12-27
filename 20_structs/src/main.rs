// Struct
struct Person {
    name: String,
    age: i32,
    weight: i32,
}

// Tuple struct
struct Color(u8, u8, u8);

// Associated functions and methods
impl Person {
    fn get_name(&self) -> &str {
        return &self.name;
    }

    fn add_weight(&mut self, diff: i32) {
        self.weight += diff;
    }

    fn new(name: String) -> Person {
        return Person {
            name: name,
            age: 20,
            weight: 70,
        };
    }
}

fn main() {
    let person = Person {
        name: String::from("Majid"),
        age: 33,
        weight: 75,
    };
    let person2 = Person {
        name: String::from("Kamali"),
        ..person
    };
    let mut person3 = Person::new(String::from("PersonName"));
    let red = Color(255, 0, 0);

    person3.add_weight(10);

    println!("person name = {}", person.name);
    println!("person2 name = {}", person2.get_name());
    println!("person3 name = {}", person3.get_name());
    println!("person3 weight = {}", person3.weight);
    println!("color red channel is {}", red.0);
}
