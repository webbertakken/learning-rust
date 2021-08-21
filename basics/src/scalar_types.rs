// Silence some warnings so they don't distract from the exercise.
#![allow(unused_variables)]

pub fn main() {
    integers();
    floats();
    booleans();
    characters();
}

fn integers() {
    // Unsigned integers
    let u8: u8;
    let u16: u16;
    let u32: u32;
    let u64: u64;
    let u128: u128;
    let usize: usize;

    // Signed integers
    let i8: i8;
    let i16: i16;
    let i32: i32;
    let i64: i64;
    let i128: i128;
    let isize: isize;

    let x = 5_u16;
}

fn floats() {
    let f32: f32;
    let f64: f64;

    let y = 3.14_f32;
}

fn booleans() {
    let true_value: bool = true;
    let false_value: bool = false;

    let boolean_as_integer = true as u8;
}

fn characters() {
    // Always 4 bytes (UCS-4 / UTF-32)
    let char: char = '🦄';
}
