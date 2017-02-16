fn main() {
    let outer = 32;
    {
        let inner = 3.14;
        println!("block variable inner: {}", inner);
        let outer = 99;
        println!("block variable outer: {}", outer);
    }
    println!("outer variable: {}", outer);
}
