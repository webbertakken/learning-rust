// Silence some warnings so they don't distract from the exercise.
#![allow(unused_variables, dead_code, unused_imports)]

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::io::Read;

pub fn main() {
    println!("\nerrors...");

    best_practices_for_errors();
    simplify_best_practices_for_errors();
    return_error_from_function();
    simplify_return_error_from_function();
}

fn best_practices_for_errors() {
    // 1. Define errors like enums.
    // 2. Use extra variants over new error types.
    // 3. Convert errors from dependencies into your own errors. (except perhaps from std library)
    // 4. Use non_exhaustive to prevent breaking changes
    // 5. Implement Debug, Display and Error traits

    #[derive(Debug)]
    #[non_exhaustive]
    pub enum PuzzleError {
        WontFit(u16),
        TooManyPieces,
        TooFewPieces,
    }

    impl Display for PuzzleError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            use PuzzleError::*;
            match self {
                WontFit(size) => {
                    write!(f, "The piece is too big for the puzzle ({}x{})", size, size)
                }
                TooManyPieces => write!(f, "The puzzle has too many pieces"),
                TooFewPieces => write!(f, "The puzzle has too few pieces"),
            }
        }
    }

    impl Error for PuzzleError {}
}

fn simplify_best_practices_for_errors() {
    // 5b. Use `thiserror` package to simplify the implementation of the error trait
    #[derive(Debug, thiserror::Error)]
    #[non_exhaustive]
    pub enum PuzzleError {
        #[error("The piece is too big or small for the puzzle ({0}x{0})")]
        WontFit(u16),
        #[error("The puzzle has too many pieces")]
        TooManyPieces,
        #[error("The puzzle has too few pieces")]
        TooFewPieces,
    }
}

fn return_error_from_function() {
    // Default syntax
    fn poem() -> Result<String, io::Error> {
        let file = match File::open("poem.txt") {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        // Do something with the file
        Ok(String::new())
    }

    // Shorter syntax, using try operator (?) which returns the error if it is not Ok
    fn poem2() -> Result<String, io::Error> {
        let file = File::open("poem.txt")?;

        // Do something with the file
        Ok(String::new())
    }
}

fn simplify_return_error_from_function() {
    // Use anyhow package to simplify the return of errors
}
