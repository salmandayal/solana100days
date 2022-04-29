/**
 * Prmitive Types ----
 * Integers : u8,i8, u16, i16,  u32, i32 by default, u64, i64, u128, i128
 * Floats : f32, f64 by default
 * Booleans : bool
 * Characters : char - unicode character
 * Tuples
 * Arrays - Fixed Length ( Vector - Dynamic Length )
 */

//Rust is statically typed language and its compiler can also infer the type at compile time

pub fn run() {
    let x = 1; //By default i32
    let y = 2.5; //By default f64

    //Add Explicit type
    let y: i64 = 922337203685477580;

    //Find max size
    println!("Max i32:{}", std::i32::MAX);
    println!("Max of i64:{}", std::i64::MAX);

    //Boolean
    let is_active = true;

    let is_greater = 10 < 5;

    let a1 = 'a'; // Single quote signifies a character - unicode character

    let face = '\u{1F600}'; //Unicode character
    println!("{:?}", (is_active, x, y, is_greater, a1,face));
}
 