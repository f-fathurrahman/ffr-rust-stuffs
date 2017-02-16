fn main() {
    println!("{} days", 31);

    // using positional arguments
    println!("{0}, this is {1}. {1} this is {0}", "Alice", "Bob");
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // using named arguments
    println!("{subject} {verb} {object}",
              object="the lazy dog",
              subject="the quick brown fox",
              verb="jumps over");

    // special formatting
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // right-align text with a specified width
    println!("{number:>width$}", number=211, width=20);
    println!("{number:>width$}", number=32, width=20);

}
