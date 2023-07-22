fn main() {
    let x = 3;
    let mut y = x;
    println!("x = {}, y = {}", x, y);
    
    y = x + 1;
    println!("x = {}, y = {}", x, y);
}


/*
Stack-only data: copy
Types such as integers that have a known size at compile time are
stored entirely on the stack, so copies of the actual values are quick to make.
That means theres's no reason we would want to prevent x from being valid after we
create the variable y.
In other words, there's no difference between deep and shallow copying here, so
calling clone wouldn't do anything different fromt he usual shallow copying, and we
can leave it out.
*/
