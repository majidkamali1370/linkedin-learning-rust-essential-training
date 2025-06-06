use std::fs;
use std::io;

fn read_and_combine(f1: &str, f2: &str) -> Result<String, io::Error> {
    let mut s1 = fs::read_to_string(f1)?; // ? is the simpler form of below syntax
    let s2 = match fs::read_to_string(f2) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };
    s1.push('\n');
    s1.push_str(&s2);
    return Ok(s1);
}

fn main() {
    let combined = read_and_combine("file1.txt", "file2.txt");
    match combined {
        Ok(s) => println!("Result is ...\n{}", s),
        Err(e) => println!("There was an error: {}", e),
    }
}
