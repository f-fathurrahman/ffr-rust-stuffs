use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn main() {
    // Fixed-size arrau
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value
    let ys: [i32; 10] = [34; 10];

    println!("xs = {:?}", xs);
    println!("ys = {:?}", ys);

    println!("xs[0] = {:?}", xs[0]);
    println!("ys[2] = {:?}", ys[2]);

    println!("xs.len() = {}", xs.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&xs));
    // each of i32 is 4 byte

    // Arrays can be automatically borrowed as slices
    println!("Borrow the whole array as a slice");
    analyze_slice(&xs);

    // Slices can point to a section of an array
    // They are of the form [starting_idx .. ending_idx]
    println!("Borros a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);
}