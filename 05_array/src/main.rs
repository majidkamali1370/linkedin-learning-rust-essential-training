fn main() {
    let letters = ['a', 'b', 'c', 'd'];
    let mut strings = ["Hello", "World", "!"];

    strings[2] = ".";

    println!("letters[0] = {}", letters[0]);
    println!("Strings = {} {} {}", strings[0], strings[1], strings[2]);

    let numbers0: [i32; 10];
    numbers0 = [0; 10]; // puts 10 zeros in the array

    println!("{}", numbers0[8]);

    let init_2d = [[1, 2, 3], [4, 5, 6]];

    println!(
        "{} {} {}\n{} {} {}",
        init_2d[0][0], init_2d[0][1], init_2d[0][2], init_2d[1][0], init_2d[1][1], init_2d[1][2]
    );
}
