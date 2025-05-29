#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

#[derive(Debug)]
enum Command {
    Clear,
    DrawLine(f64, f64),
    DrawShape(Shape),
}

fn main() {
    let my_shape = Shape::Triangle(3.0, 4.0, 5.0);
    println!("My shape is {:?}", my_shape);

    let my_command_1 = Command::Clear;
    let my_command_2 = Command::DrawShape(my_shape);
    println!("My command 1 is {:?}", my_command_1);
    println!("My command 1 is {:?}", my_command_2);
}
