fn main() {
    let message = "Hello, world from me!";
    let world = &message[7..12];
    let last_part = &message[13..];

    println!("last word in message is '{world}'");
    println!("last_part is '{last_part}'");

    let planets = [
        "Mercury", "Venus", "Earth", "Mars", "Jupiter", "Saturn", "Uranus", "Neptune",
    ];
    let inner_planets = &planets[..4];

    println!("inner planets = {:?}", inner_planets);

    println!(
        "first word of '{message}' is '{}'",
        find_first_word(&message)
    );
    println!(
        "first word of '{}' is '{}'",
        &message[7..],
        find_first_word(&message[7..])
    );
}

fn find_first_word(s: &str) -> &str {
    let b = s.as_bytes();

    for (index, &item) in b.iter().enumerate() {
        if item == b' ' {
            return &s[..index];
        }
    }

    return &s;
}
