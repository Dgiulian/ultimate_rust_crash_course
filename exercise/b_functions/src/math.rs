pub fn volume(x: i32, y: i32, z: i32) -> i32 {
    x * y * z
}

pub fn area_of(x: i32, y: i32) -> i32 {
    // 2a. Fix this function to correctly compute the area of a rectangle given
    // dimensions x and y by multiplying x and y and returning the result.
    //
    x * y
    // Challenge: It isn't idiomatic (the normal way a Rust programmer would do things) to use
    //            `return` on the last line of a function. Change the last line to be a
    //            "tail expression" that returns a value without using `return`.
    //            Hint: `cargo clippy` will warn you about this exact thing.
}
