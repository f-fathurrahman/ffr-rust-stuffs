/*
Mutable reference have one big restriction: if you have a mutable reference
to a value, you can have no other references to that value.
*/

fn main() {
    let mut s = String::from("hello");
    println!("s = {}", s);

    let r1 = &mut s;
    //println!("r1 = {}", r1);

    // cannot do this
    //let r2 = &mut s;

    // can do this ?
    //let r2 = s.clone();
    //println!("r2 = {}", r2);

    r1.push_str(" efefer");
    println!("s = {}", s);
}