fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s; // s is borrowed by r1
    println!("r1 = {}", r1);
    // r1 usage is finished here, ownership transferred after call to println!

    let r2 = &mut s; // s is borrowed by r2
    println!("r2 = {}", r2);
}
