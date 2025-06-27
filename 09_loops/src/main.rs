fn main() {
    let mut count = 0;

    // Infinite loop, must be explicitly break (optionally with a value)
    let result = loop {
        count += 1;
        println!("LOOP => count is {count}");

        if count == 10 {
            break count * 3;
        }
    };

    println!("result = {result}");

    // While loop, must be explicitly break
    while count <= 20 {
        println!("WHILE => count is {count}");
        count += 1; // There's no ++ operator in Rust
    }

    let items = [1, 2, 3, 4, 5];

    // For loop, for each item in collection
    for item in items {
        println!("FOR => item is {item}");
    }

    // For loop, for each item in range
    // Use .. to represent a range of numbers. Operands can be either literals or variables
    for number in 0..10 {
        println!("number is {number}");
    }
}
