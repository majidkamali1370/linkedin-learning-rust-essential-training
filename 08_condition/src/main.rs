fn main() {
    let x = 4;

    if x > 5 {
        println!("x is more than 5");
    }

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

    let y = if x < 5 { x + 5 } else { x };

    println!("y is {y}");

    // if x {} // compile error
    // if x as bool {} // compile error
}
