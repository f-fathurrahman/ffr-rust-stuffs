fn main() {
    let s1 = String::from("hello from efefer");
    let (s2, nchars) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, nchars);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    return (s, length);
}