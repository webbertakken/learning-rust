// Silence some warnings so they don't distract from the exercise.
#![allow(
    unused_variables,
    dead_code,
    unused_must_use,
    unused_assignments,
    unreachable_patterns
)]

pub fn main() {
    println!("\nenums...");

    different_kinds_of_items_in_an_enum();
    the_option_enum();
    the_result_enum();
    using_generics_and_matching_enum_values();
}

// Super basic enum
enum Color {
    Red,
    Green,
    Blue,
}

// A more interesting enum
enum DispenserItem {
    Empty,
    Ammo(u8),
    Things(String, i32),
    Place { x: i32, y: i32 },
}

// Implement methods for an enum (this is pretty cool)
impl DispenserItem {
    fn display(&self) -> String {
        String::from("Display not implemented yet")
    }
}

fn different_kinds_of_items_in_an_enum() {
    use DispenserItem::*;
    let item = Place { x: 24, y: 48 };

    item.display();
}

fn the_option_enum() {
    // Generic enum, this one will be used frequently (exists natively).
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    // used like this: (by default imported from std library)
    let mut x: Option<i32> = None;

    // Setting x
    x = Some(2);

    // Fast checks
    x.is_some(); // true
    x.is_none(); // false

    // Using iterator trait
    for i in x {
        println!("For loop returns value from enum: {}", i); // "2"
    }
}

fn the_result_enum() {
    // #[must_use]
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    use std::fs::File;

    let res = File::open("foo");
    match res {
        Ok(f) => { /* do stuff */ }
        Err(e) => { /* do stuff */ }
    }
}

fn using_generics_and_matching_enum_values() {
    let my_variable: Option<String> = Some(String::from("myVariableString"));

    // Matching a single case
    if let Some(ref x) = my_variable {
        println!("Single value is {}", x);
    }

    // Matching all cases
    // Match expression must be exhaustive: requires writing a branch arm for each enum field
    match my_variable {
        Some(x) => {
            println!("Multi-match value is {}", x);
        }
        None => {
            println!("no value");
        }
        // The underscore (_) counts as default or anything-else branch
        _ => {
            println!("Not implemented");
        }
    }

    // Or shorter
    let my_variable = Some(3);
    let x = match my_variable {
        Some(x) => x * x,
        None => 0,
    };
}
