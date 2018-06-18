mod gcd;

// a comment
fn main() {
    say_hello();
    driver_gcd();

    println!("\nThis is the end of main")
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
