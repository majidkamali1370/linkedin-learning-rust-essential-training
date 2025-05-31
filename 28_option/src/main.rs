fn get_number(condition: bool, value: i32) -> Option<i32> {
    if condition { Some(value * 5) } else { None }
}

fn main() {
    let maybe_number_1 = get_number(true, 12);
    let maybe_number_2 = get_number(false, 1);
    let number = maybe_number_1.unwrap_or(5) + 1;
    let another_number = match maybe_number_1 {
        Some(maybe_number_1) => maybe_number_1,
        None => 100,
    };

    println!("number1 is {:?}", maybe_number_1);
    println!("number2 is {:?}", maybe_number_2);
    println!("number is {:?}", number);
    println!("another number is {:?}", another_number);
}
