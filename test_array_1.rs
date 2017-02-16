use std::mem;

fn analyze_slice( slice: &[i32] ) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn main() {
    // Fixed-size array (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 4, 5, 7];

    // All elements can be initialized to the same value
    let ys: [i32; 500] = [42; 500];

    // Indexing starts at 0
    println!("\nFirst element of the array xs: {}", xs[0]);
    println!("Second element of the array xs: {}", xs[1]);

    println!("\nFirst element of the array ys: {}", ys[0]);
    println!("Second element of the array ys: {}", ys[1]);

    // `len` returns the size of the array
    println!("\nArray size xs: {}", xs.len());
    println!("\nArray size ys: {}", ys.len());

    // Arrays are stack allocated
    println!("\nArray xs occupies {} bytes", mem::size_of_val(&xs));
    //println!("Array ys occupies {} bytes", mem:size_of_val(&ys));

    // Arrays can be automatically borrowed as slices
    println!("\nBorrow the whole array as a slice");
    analyze_slice(&xs);

    // Slices can point to a section of an array
    println!("\nBorrow a section of the array as a slice");
    analyze_slice(&ys[1..4]);
}
