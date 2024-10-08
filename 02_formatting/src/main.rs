fn main() {
    let a = 10.0;
    let b = 3.0;
    let c = a / b;
    let my_string = "Hello world";

    println!("c is {:.3}", c);
    println!("c is {:8.3}", c);
    println!("c is {:+8.3}", c);
    println!("c is {:+08.3}", c);
    println!("a is {2}, b is {1} c is {0:.3}", c, b, a);
    println!("Sentence is '{my_string}'");
}
