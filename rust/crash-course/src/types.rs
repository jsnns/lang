/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits unsiged/integer)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

// Rust is statically typed, compiler can usually infer what type we want to use based on value
// and how we use it

pub fn run() {
    // default is i32
    let x = 1;

    // default is f64
    let y = 2.5;

    // add explicit to change size
    let z: i8 = 10;

    // find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i32: {}", std::u32::MAX);

    // boolean
    let is_active: bool = true;

    // boolean from experesion
    let is_greater = 10 > 5;

    // char is single-quotes
    let a1 = 'a';

    // unicode smiley
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face))
}
