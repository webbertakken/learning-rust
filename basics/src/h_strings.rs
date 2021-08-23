pub fn main() {
    println!("\nstrings...");

    string_slice()
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
