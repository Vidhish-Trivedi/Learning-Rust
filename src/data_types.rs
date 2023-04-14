/* Primitive Types:
            Integers: u8, i8, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory).
            Floats: f32, f64.
            Boolean (bool)
            Characters (char)
            Tuples
            Arrays  (fixed length)
*/

// Rust is a statically and strongly-typed language, which means that it must know the types of all variables
// at compile time, however, the compiler can usually infer what type we want to use based on the value and how we use it.

pub fn run() {
    // By default, the inferred type would be i32.
    let x = 1;

    // By default, the inferred type is f64.
    let y = 2.5;

    // Explicit typing.
    let z: i64 = 3113409188;

    // Get max or min size of a datatype.
    println!("Max size of i32 is: {}", std::i32::MAX);
    println!("Min size of i64 is: {}", std::i64::MIN);

    // Boolean.
    let is_active = true;
    let is_bad: bool = false;

    // Get boolean from expressions.
    let is_greater: bool = 3 > 5;

    // Characters.      UNICODE.
    let a1 = 'a';               // Note that we have used single-quotes here.
    let face = '\u{1F600}';     // We can pass in the unicode for emojis.

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}
