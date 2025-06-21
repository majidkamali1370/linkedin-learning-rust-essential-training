fn main() {
    let a = 10.0;
    let b = 3.0;
    let c = a / b;
    let my_string = "Hello world";

    println!("c is {:.3}", c); // Show 3 digits after decimal point
    println!("c is {:8.3}", c); // Print the value using width of 8 characters
    println!("c is {:+8.3}", c); // Shw plus sign
    println!("c is {:+08.3}", c); // Fill left with zero
    println!("a is {2}, b is {1} c is {0:.3}", c, b, a); // Reorder variables
    println!("Sentence is '{my_string}'"); // Can use variable name inside string
}
