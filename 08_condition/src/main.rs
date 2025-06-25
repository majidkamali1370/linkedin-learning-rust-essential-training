fn main() {
    let x = 4;

    // There's no () around conditions
    if x > 5 {
        println!("x is more than 5");
    }

    // Even a single-line body, has to have {}
    if x != 5 {
        println!("x is not 5");
    } else {
        println!("x is 5");
    }

    if x > 10 {
        println!("x > 10");
    } else if x == 10 {
        println!("x == 10");
    } else {
        println!("x < 10");
    }

    let y = if x < 5 { x + 5 } else { x }; // Conditional intialization

    println!("y is {y}");

    // if x {} // compile error
    // if x as bool {} // compile error
}
