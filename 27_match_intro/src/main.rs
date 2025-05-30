#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

fn main() {
    let my_shape = Shape::Rectangle(1.2, 4.6);
    println!("My shape is {:?}", my_shape);

    match my_shape {
        Shape::Circle(r) => println!("Circle with radius {r}"),
        Shape::Rectangle(w, h) => println!("{w} x {h} Rectangle"),
        Shape::Triangle(a, b, c) => println!("Triangle with sides {a}, {b}, {c}"),
    }

    let my_variable = 5u8;
    let result = match my_variable {
        0 => "zero",
        1 => "one",
        2 => "two",
        _ => {
            println!("Number did not match");
            "other numbers"
        }
    };
    println!("number is {result}");
}
