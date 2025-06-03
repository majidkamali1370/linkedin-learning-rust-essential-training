fn main() {
    let number = Some(12);

    match number {
        Some(12) => println!("Twelve from match"),
        _ => (),
    }

    if let Some(12) = number {
        println!("Twelve from if-let");
    }
}
