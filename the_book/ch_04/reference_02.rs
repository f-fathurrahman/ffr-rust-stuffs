fn main() {
    let mut s = String::from("hello");
    println!("Before change: s = {}", s);
    
    change(&mut s);
    println!("After change s = {}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world from efefer");
}
