// Tuples can be used as function arguments and as return values
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = pair;
    return (bool_param, int_param);
} 


fn main() {
    let pair = (5, true);
    println!("pair is {:?}", pair);
    println!("reversed pair: {:?}", reverse(pair));
}