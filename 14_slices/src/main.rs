fn main() {
    let message = "Hello, world from me!";
    let world = &message[7..12]; // Slice of string, from index 7 (including) to index 12 (excluding)
    let last_part = &message[13..]; // Slice of string from index 13 onwards

    println!("last word in message is '{world}'");
    println!("last_part is '{last_part}'");

    let planets = [
        "Mercury", "Venus", "Earth", "Mars", "Jupiter", "Saturn", "Uranus", "Neptune",
    ];
    let inner_planets = &planets[..4]; // Slice of array, upto index 4 (excluding)

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
    let b = s.as_bytes(); // Convert string slice to byte array

    // Enumerate over the bytes using enumerate function of iterator
    for (index, &item) in b.iter().enumerate() {
        if item == b' ' {
            return &s[..index]; // Returning a slice
        }
    }

    return &s;
}
