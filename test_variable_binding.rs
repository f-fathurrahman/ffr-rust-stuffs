fn main() {
    let an_integer = 3u32;
    let a_boolean = true;
    let unit = ();

    let copied_integer = an_integer;

    println!("an_integer: {}", an_integer);
    println!("copied_integer: {}", copied_integer);
    println!("A boolean: {}", a_boolean);
    println!("Meet the unit value: {:?}", unit);
}
