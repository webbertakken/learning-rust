pub fn main() {
    println!("\niterators...");

    iterator_generators()
}

fn iterator_generators() {
    let mut v = vec![1, 2, 3, 4, 5];

    // Return immutable references to the elements of the vector.
    let immutableReferences = v.iter();
    // The same as
    for _ in &v {}

    // Return mutable references to the elements of the vector.
    let mutableReferences = v.iter_mut();
    // The same as
    for _ in &mut v {}

    // Consume the vector and return the elements as owned items.
    let ownedItems = v.into_iter();
    // The same as
    for _ in ownedItems {} // v was consumed by v.into_iter()
}
