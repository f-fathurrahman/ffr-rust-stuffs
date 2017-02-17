fn main() {
    fn function(i: i32) -> i32 {i + 1}

    let closure_annotated = |i: i32| -> i32 { i + 1 };
    // wil result in compile error if not called
    let closure_inferred  = |i| i + 1.1;

    let i = 1;

    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(2.3));

    let one = || 1.321f64;
    println!("closure returning some number: {}", one());
}
