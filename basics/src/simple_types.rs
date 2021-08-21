// Silence some warnings so they don't distract from the exercise.
#![allow(unused_variables)]

use basics::print_difference;

pub fn main() {
    println!("\nsimple types...");

    use_some_simple_types();
}

fn use_some_simple_types() {
    let coords: (f32, f32) = (6.3, 15.0);
    print_difference(coords.0, coords.1);

    let coords_array: [f32; 2] = [coords.0, coords.1];
    print_array(coords_array);

    let series = [1, 1, 2, 3, 5, 8, 13];
    ding(series[series.len() - 1]);

    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
    on_off(mess.2[1].0);

    print_distance(coords);
}

fn print_array(a: [f32; 2]) {
    println!("The coordinates are ({}, {})", a[0], a[1]);
}

fn ding(x: i32) {
    if x == 13 {
        println!("Ding, you found 13!");
    }
}

fn on_off(val: bool) {
    if val {
        println!("Lights are on!");
    }
}

fn print_distance(z: (f32, f32)) {
    // Using z.0 and z.1 is not nearly as nice as using x and y.  Lucky for
    // us, Rust supports destructuring function arguments.  Try replacing "z" in
    // the parameter list above with "(x, y)" and then adjust the a function
    // body to use x and y.
    println!(
        "Distance to the origin is {}",
        (z.0.powf(2.0) + z.1.powf(2.0)).sqrt()
    );
}
