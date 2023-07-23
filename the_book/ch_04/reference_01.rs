fn main() {
    let s1 = String::from("hello");
    let nchars = calculate_length(&s1);
    /*
    The ampersands represents references, and they allow you to refer
    to some value without taking ownership of it.
    */

    println!("The length of '{}' is {}.", s1, nchars);
}

fn calculate_length(s: &String) -> usize {
    return s.len();
}
// Here, s goes out of scope. But it does not have ownership of what it refers to
// so it is not dropped.
