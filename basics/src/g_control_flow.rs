// Silence some warnings so they don't distract from the exercise.

pub fn main() {
    println!("\ncontrol flow...");

    exercise();
    variable_from_result_of_if_statement();
    breaking_out_of_nested_loops_using_labels();
    a_while_using_another_function_as_a_condition();
    a_for_loop_using_an_iterable();
    a_for_loop_using_a_range();
    a_for_loop_destructuring_tuples_in_an_array();
}

fn exercise() {
    // This collects any command-line arguments into a vector of Strings.
    // For example:
    //
    //     cargo run apple banana
    //
    // ...produces the equivalent of
    //
    //     vec!["apple".to_string(), "banana".to_string()]
    let args: Vec<String> = std::env::args().skip(1).collect();

    // This consumes the `args` vector to iterate through each String
    for arg in args {
        if arg == "sum" {
            sum();
        } else if arg == "double" {
            double();
        } else {
            count(arg);
        }
    }
}

fn sum() {
    let mut sum = 0;

    for current in 7..=23 {
        sum += current;
    }

    println!("The sum is {}", sum);
}

fn double() {
    let mut count = 0;
    let mut x = 1;

    while x <= 500 {
        x += x;

        count += 1;
    }

    println!(
        "You can double x {} times until x is larger than 500",
        count
    );
}

fn count(arg: String) {
    let mut times = 0;
    loop {
        if times >= 8 {
            break;
        }

        print!("{} ", arg);
        times += 1;
    }

    println!();
}

fn variable_from_result_of_if_statement() {
    let result = if true == true {
        "some value"
    } else {
        "some other value"
    };

    println!("{}", result);
}

#[allow(unused_assignments)]
fn breaking_out_of_nested_loops_using_labels() {
    let mut innermost_loop_reached: bool = false;

    'outermost_loop: loop {
        loop {
            loop {
                innermost_loop_reached = true;
                break 'outermost_loop;
            }
        }
    }

    println!(
        "Inner loop was {}reached",
        if innermost_loop_reached { "" } else { "not " }
    )
}

fn a_while_using_another_function_as_a_condition() {
    let mut energy: i32 = 0;

    fn tired(energy: i32) -> bool {
        return energy <= 6;
    }

    fn rest(energy: i32) -> i32 {
        return energy + 1;
    }

    while tired(energy) {
        energy = rest(energy);
    }
}

fn a_for_loop_using_an_iterable() {
    for number in [1, 2, 3].iter() {
        print!("{}, ", number);
    }

    println!();
}

fn a_for_loop_using_a_range() {
    // The equals sign is for inclusive range
    for number in 1..=10 {
        print!("{}, ", number);
    }

    println!();
}

fn a_for_loop_destructuring_tuples_in_an_array() {
    for (a, b) in [(1, 2), (3, 4)].iter() {
        print!("{} and {}, ", a, b);
    }

    println!();
}
