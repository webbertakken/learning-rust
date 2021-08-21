// Silence some warnings so they don't distract from the exercise.
#![allow(unused_variables)]

pub fn main() {
    tuples();
    arrays();
}

fn tuples() {
    let tuple = (1, 3.3, 999);

    let member_one = tuple.0;
    let member_two = tuple.1;
    let member_three = tuple.2;

    // Destructuring
    let (one, two, three) = tuple;

    // Tuples have a maximum arity of 12 - above which they lose functionality.
}

pub fn arrays() {
    let buffer_one = [1, 2, 3];
    let buffer_two = [0; 3];
    let buffer_three: [u8; 3] = [1, 2, 3];

    // Arrays are limited to a size of 32 - above which they lose functionality.
    // This is because they live on the stack by default and are fixed size.

    // You'll usually want to use vectors or slices of vectors instead of arrays.
}
