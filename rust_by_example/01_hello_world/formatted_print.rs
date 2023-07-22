fn main() {
    // The `{}` will be automatically replaced with any arguments.
    // These will be stringified.
    println!("{} days", 31);

    // Positional arguments by specifying an integer inside `{}`
    println!("{0}, this is {1}, {1}, this is {0}", "Alice", "Bob");

    // Named arguments
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Different formatting
    println!("Base 10 (decimal):                  {}", 1234567);
    println!("Base 2 (binary):                    {:b}", 1234567);
    println!("Base 8 (octal):                     {:o}", 1234567);
    println!("Base 16 (binary):                   {:x}", 1234567);
    println!("Base 16 (hexadecimal) with capital: {:X}", 1234567);

}

