fn main() {
    let s = String::from("hello"); // s come into scope
    
    //take_ownership(s); // s's value moves into the function ...
    // ... and so it is no longer valid here
    
    take_ownership(s.clone());
    println!("in main: s = {}", s);

}

fn take_ownership(some_string: String) { // some_string comes into scope
    println!("in take_ownership: {}", some_string);
}
// Here, some_string goes out of scope and `drop` is called.
// The backing memory is freed.


