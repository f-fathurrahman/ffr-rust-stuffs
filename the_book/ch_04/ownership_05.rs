fn main() {
    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function
    // but i32 is Copy, so it is okay to still use x afterward
    println!("x = {}", x);
}

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("some_integer in makes_copy: {}", some_integer);
}
// Here, some_integer goes out of scope. Nothing special happens.
