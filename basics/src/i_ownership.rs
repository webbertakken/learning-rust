// Silence some warnings so they don't distract from the exercise.
#![allow(unused_variables)]

pub fn main() {
    println!("\nownership...");

    move_value_to_new_pointer();
    copy_value_to_new_pointer();
    changing_values_prevention_in_side_effects();
    changing_values_correct_signature_without_using_reference();
    changing_values_using_immutable_references();
    changing_values_using_mutable_references();
}

fn move_value_to_new_pointer() {
    let s1 = String::from("abc");
    //  -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait

    let s2 = s1;
    //             -- value moved here (s1 no longer points to the value)

    // println!("{}", s1);
    // Error:         ^^ value borrowed here after move
}

fn copy_value_to_new_pointer() {
    let s1 = String::from("abc");
    let s2 = s1.clone();

    println!("{} and {}", s1, s2);
    // Outputs: "abc and abc"
}

fn changing_values_prevention_in_side_effects() {
    let s1 = String::from("abc");

    side_effect(s1);

    // println!("{}", s1);
    // Error:         ^^ value borrowed here after move

    fn side_effect(string: String) {
        // Do stuff with the string
    }
}

fn changing_values_correct_signature_without_using_reference() {
    let mut s1 = String::from("foo");

    s1 = function_that_returns_modified_string(s1);

    println!("{}", s1); // Prints "foo"

    fn function_that_returns_modified_string(string: String) -> String {
        string
    }
}

fn changing_values_using_immutable_references() {
    let s1 = String::from("bar");

    // pass s1 by (immutable) reference
    side_effect(&s1);

    println!("{}", s1); // Prints "bar"

    fn side_effect(string: &String) {
        // Do stuff
    }
}

fn changing_values_using_mutable_references() {
    let mut s1 = String::from("baz");

    // pass s1 by (mutable) reference
    function_that_updates_string(&mut s1);

    println!("{} after having been updated", s1); // Prints "baz"

    fn function_that_updates_string(string: &mut String) {
        // The * gives mutable access to the variables value.
        *string = string.to_string();
    }
}
