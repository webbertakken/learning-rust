pub fn main() {
    println!("basic concepts...");
    scoping();
    shadowing();
    make_immutable_using_shadowing();
    changing_data_type_like_in_data_transformation_pipelines();
}

fn scoping() {
    let x = 5;
    {
        let y = 99;
        println!("{},{}", x, y); // Prints "5,99"
    }
    // y gets dropped immediately after scope ends

    println!("{}", x) // Prints "5"
}

fn shadowing() {
    let x = 5;
    {
        // Subsequent let will shadow previous let
        let x = 99;
        println!("{}", x); // Prints "99"
    }

    println!("{}", x) // Prints "5"
}

#[allow(unused_variables, unused_mut)]
fn make_immutable_using_shadowing() {
    let mut x = 0; // x is mutable

    // This line of code often does not even show up in the assembly code,
    // depending on the compilers optimisation path.
    let x = x; // x is now immutable
}

#[allow(clippy::let_and_return)]
fn changing_data_type_like_in_data_transformation_pipelines() -> usize {
    let changing_type = "StartAsAString";

    // As the intermediate variable (the string) is never exposed, so changing its type might be
    // useful for specific cases like data transformation pipes.
    let changing_type = changing_type.len();

    changing_type
}
