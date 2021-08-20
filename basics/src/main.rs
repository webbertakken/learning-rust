fn main() {
    scoping();
    shadowing();
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

