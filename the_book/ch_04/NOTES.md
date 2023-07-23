```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);
```

Compile and then error:
```
.... // error message

For more information about this error, try `rustc --explain E0499`.
error: could not compile `ownership` due to previous error
```

This error says that this code is invalid because we cannot borrow `s` as mutable
more than once at a time.
The first mutable borrow is in `r1` and must last until it's used in the `println!`,
but between the creation of that mutable reference and its usage, we tried to create
another mutable reference in `r2` that borrows the same data as `r1`.