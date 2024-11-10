fn main() {
    let mut count = 0;

    let result = loop {
        count += 1;
        println!("LOOP => count is {count}");

        if count == 10 {
            break count * 3;
        }
    };

    println!("result = {result}");

    while count <= 20 {
        println!("WHILE => count is {count}");
        count += 1;
    }

    let items = [1, 2, 3, 4, 5];

    for item in items {
        println!("FOR => item is {item}");
    }

    for number in 0..10 {
        println!("number is {number}");
    }
}
