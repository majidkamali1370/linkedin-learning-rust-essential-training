fn main() {
    let my_variable = "Hello World";
    println!("my_variable = {my_variable}");
    let my_variable = "New String!";
    println!("my_variable = {my_variable}");

    {
        let my_variable = 10;
        println!("my_variable = {my_variable}");
    }

    {
        let mut my_variable = 10;
        my_variable += 1;
        println!("my_variable = {my_variable}");
    }

    println!("my_variable = {my_variable}");
}
