fn main() {
    let item = (20, "Hello", 3.14); // Use () to create a tuple, just like python
    let another_item: (i32, f64, char);
    another_item = (100, 3.14, 'X'); // You can write const variable only once

    println!("{}, {}, {}", item.0, item.1, item.2); // Access .# to access tuple items
    println!("{}, {}, {}", another_item.0, another_item.1, another_item.2);
    println!("another_item = {:?}", another_item);

    // Use _ to ignore a variable (_ = placeholder)
    let (number1, number2, _) = another_item; // Unpack tuple using ()

    println!("number1 is {}, number2 is {}", number1, number2);
}
