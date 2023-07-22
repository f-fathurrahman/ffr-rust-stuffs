fn main() {
    // tuple with various types
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
        -1i8, -2i16, -3i32, -4i64,
        0.1f32, 0.2f64,
        'a', true);

    // using tuple indexing
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);

    println!("This is long_tuple: {:?}", long_tuple);

    // Long tuples with more than 12 elements cannot be printed (?)
    /*
    let too_long_tuple = (1,2,3,4,5,6,7,8,9,10,11,12,13);
    println!("too_long_tuple: {:?}", too_long_tuple);
    */

}

