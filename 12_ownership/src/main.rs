fn main() {
    {
        let i = 10;
        println!("i = {i}");
        copy_stack_based_value(i); // Copy i32 value to function parameter
        println!("i = {i}\n");
    }

    {
        let s = String::from("Hello");
        println!("s = {s}");
        copy_heap_based_value(s.clone()); // Copy heap-based value to function parameter
        println!("s = {s}\n");
    }

    {
        let s = String::from("Hello");
        println!("s = {s}");
        move_heap_based_value(s); // Move heap-based value to function parameter
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

// Pass by value
fn copy_stack_based_value(mut v: i32) {
    v += 1;
    println!("v = {v}");
}

// Pass by move. Note that function signature is like pass by value, but because variable s on line 12 is cloned,
// this function grabs a moved-from value of the copied object
fn copy_heap_based_value(mut v: String) {
    v.push_str(" World");
    println!("v = {v}");
}

// Pass by move
fn move_heap_based_value(mut v: String) {
    v.push_str(" World");
    println!("v = {v}");
}

// Pass by move, return the same variable by move
fn move_heap_based_value_and_return(mut v: String) -> String {
    v.push_str(" World");
    println!("v = {v}");
    return v;
}
