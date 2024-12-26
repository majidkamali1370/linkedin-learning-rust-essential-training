use std::fs;
use std::io::Write;

fn main() {
    {
        // read as String
        let content = fs::read_to_string("planets.txt").unwrap();
        println!("Content is {:?}", content);

        for line in content.lines() {
            println!("line is {}", line);
        }
    }

    {
        // read as bytes
        let content = fs::read("planets.txt").unwrap();
        println!("Content is {:?}", content);
    }

    {
        // write string
        let mut string = String::new();
        string.push_str("This\n");
        string.push_str("is\n");
        string.push_str("a\n");
        string.push_str("string.");

        fs::write("output.txt", string);
    }

    {
        // append
        let mut file = fs::OpenOptions::new()
            .append(true)
            .open("planets.txt")
            .unwrap();
        file.write(b"Pluto\n");
    }
}
