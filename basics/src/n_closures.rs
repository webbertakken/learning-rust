// Silence some warnings so they don't distract from the exercise.
#![allow(unused_variables, dead_code)]

pub fn main() {
    println!("\nclosures...");

    syntax();
    usage_with_iterators();
}

fn syntax() {
    // Basic example
    let add = |x, y| x + y;
    println!("{}", add(1, 2)); // prints "3"

    // Can also borrow for parent scope
    let x = "✔".to_string();
    let add = || println!("{}", x);
    add(); // prints "✔"
}

fn usage_with_iterators() {
    let mut vector = vec![1, 2, 3, 5];

    let result = vector
        .iter()
        .map(|x| x * 3)
        // Note the * for comparing the value instead of the reference
        .filter(|x| *x > 10)
        .fold(0, |acc, x| acc + x);

    println!("{}", result); // prints "15"
}
