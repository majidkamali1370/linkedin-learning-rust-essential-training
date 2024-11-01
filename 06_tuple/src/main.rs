fn main() {
    let item = (20, "Hello", 3.14);
    let another_item: (i32, f64, char);
    another_item = (100, 3.14, 'X');

    println!("{}, {}, {}", item.0, item.1, item.2);
    println!("{}, {}, {}", another_item.0, another_item.1, another_item.2);
    println!("another_item = {:?}", another_item);

    let (number1, number2, _) = another_item;

    println!("number1 is {}, number2 is {}", number1, number2);
}
