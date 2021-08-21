﻿// Silence some warnings so they don't distract from the exercise.
#![allow(unused_variables)]

pub fn main() {
    println!("\nfunctions...");

    use_some_functions();
}

fn use_some_functions() {
    let width = 4;
    let height = 7;
    let depth = 10;

    {
        let area = area_of(width, height);
        println!("Area is {}", area);
    }

    println!("Volume is {}", volume(width, height, depth));
}

fn area_of(x: i32, y: i32) -> i32 {
    x * y
}

fn volume(width: i32, height: i32, depth: i32) -> i32 {
    width * height * depth
}
