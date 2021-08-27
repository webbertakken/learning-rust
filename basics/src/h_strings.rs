pub fn main() {
    println!("\nstrings...");

    string_slice();
    get_nth_character();
}

#[allow(unused_variables)]
fn string_slice() {
    // A string literal is always a "borrowed string slice (&str)"
    let string_literal: &str = "Hello 🌍";

    // A string
    let string_from_string_slice: String = String::from(string_literal);
    let string_from_string_slice_to_string: String = string_literal.to_string();

    // A borrowed string slice can not be modified, whereas a string (String) can be modified.
}

fn get_nth_character() {
    let string_of_characters: String = String::from("hørses");

    // This can not be done, because non-ascii characters use multiple bytes.
    // let third_character = string_of_characters[2];

    let third_character = string_of_characters.chars().nth(2).unwrap();

    println!(
        "{} is the third character in {}",
        third_character, string_of_characters
    );
    // prints "r is the third character in hørses"
}
