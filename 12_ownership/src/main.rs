fn main() {
    let i = 10;
    println!("i = {i}");
    copy_stack_based_value(i);
    println!("i = {i}\n");

    {
        let s = String::from("Hello");
        println!("s = {s}");
        copy_heap_based_value(s.clone());
        println!("s = {s}\n");
    }

    {
        let s = String::from("Hello");
        println!("s = {s}");
        move_heap_based_value(s);
        // println!("s = {s}"); // DOES NOT COMPILE. BECAUSE OF OWNERSHIP TRANSFER
        println!("Cannot use s variable anymore\n");
    }

    {
        let s = String::from("Hello");
        println!("s = {s}");
        let s = move_heap_based_value_and_return(s); // USE SHADOWING TO PASS THE MOVED-FROM VARIABLE AGAIN
        println!("s = {s}\n");
    }
}

fn copy_stack_based_value(mut v: i32) {
    v += 1;
    println!("v = {v}");
}

fn copy_heap_based_value(mut v: String) {
    v.push_str(" World");
    println!("v = {v}");
}

fn move_heap_based_value(mut v: String) {
    v.push_str(" World");
    println!("v = {v}");
}

fn move_heap_based_value_and_return(mut v: String) -> String {
    v.push_str(" World");
    println!("v = {v}");
    return v;
}
