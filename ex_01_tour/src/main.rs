mod gcd;

// a comment
fn main() {
    say_hello();
    driver_gcd();

    handle_command_line();

    println!("\nThis is the end of main")
}

use std::io::Write;
use std::str::FromStr;

fn handle_command_line()
{
    println!("\nEntering handle_command_line\n");

    let mut numbers = Vec::new();

    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("Error parsing argument"));
    }

    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
        std::process::exit(1);
    }

    let mut d = numbers[0];
    // calculate GCD for all numbers
    for m in &numbers[1..] {
        d = gcd::gcd(d,*m);
    }
    /*
    The	& operator in &numbers[1..] borrows a reference to the
    vector's elements from the second onward. The for loop iterates over the
    referenced elements, letting m borrow each element in succession.
    The	* operator in *m dereferences m, yielding the value it refers to;
    this is the next u64 we want to pass to gcd
    */
    println!("The greatest common divisor of {:?} is {}", numbers, d);
    println!("\nLeaving handle_command_line\n");
}


fn say_hello() {
    println!("\nEntering function say_hello");
    println!("\nHello, world!");
    println!("\nLeaving function say_hello");
}

fn driver_gcd() {
    println!("\nEntering function driver_gcd");
    println!("\ngcd(100,40) = {}", gcd::gcd(100,40));
    println!("\nLeaving function driver_gcd");
}
