/*
Mutable reference have one big restriction: if you have a mutable reference
to a value, you can have no other references to that value.
*/

fn main() {
    let mut s = String::from("hello");
    println!("s = {}", s);

    // Introduce new scope, allowing for multiple mutable reference, just not simultaneous one
    {
        let r1 = &mut s;
        println!("r1 = {}", r1);
    }

    let r2 = &mut s;

    r2.push_str(" efefer");
    println!("r2 = {}", r2);
    println!("s = {}", s);
    //println!("r2 = {}", r2); // cannot do this
}