fn gives_ownership() -> String {
    let some_string = String::from("yours");
    // some_string comes into scope
    
    return some_string;
    // some_string is returned and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    
    return a_string;
    // a_string is returned and moves out to the calling function
}


fn main() {
    let s1 = gives_ownership();
    println!("s1 = {}", s1);
    // gives_ownership moves its return value into s1
    
    let s2 = String::from("hello"); // s2 comes into scope
    println!("s2 = {}", s2);

    let s3 = takes_and_gives_back(s2);
    // s2 is moved into takes_and_gives_back, which also moves its return
    // value into s3
    println!("s3 = {}", s3);

    println!("s1 = {}", s1);
    
    // This will give compile error if we don't pass s2.clone() to takes_and_gives_back
    // println!("s2 = {}", s2);


}
// Here, s3 goes out of scope and is dropped.
// s2 was moved, so nothing happens.
// s1 goes out of scope and is dropped.
