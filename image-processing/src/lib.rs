use anyhow::{anyhow, Error, Result};
use std::str::FromStr;

pub trait Exit {
    fn print_and_exit(self) -> !;
}

impl Exit for Error {
    fn print_and_exit(self) -> ! {
        eprintln!("{:?}", self);
        std::process::exit(1);
    }
}

pub fn try_parse<T: FromStr>(value: &str) -> Result<T, Error> {
    match value.parse::<T>() {
        Ok(value) => Ok(value),
        Err(_) => Err(anyhow!("\"{}\" is not a valid value.", value)),
    }
}

pub fn parse_between<T>(value: &str, min_amount: T, max_amount: T) -> Result<T, Error>
where
    T: FromStr + PartialOrd + std::fmt::Display,
{
    match try_parse::<T>(value) {
        Ok(amount) => {
            if amount > max_amount || amount < min_amount {
                return Err(anyhow!(
                    "Expected value to be between {} and {} but received {}",
                    min_amount,
                    max_amount,
                    amount
                ));
            };

            Ok(amount)
        }
        Err(error) => Err(error),
    }
}
