use codereview_prime as prime;
use std::env;
use std::io::stdin;

use std::convert::From;
use std::fmt;

const HELP_TEXT: &str = "USAGE:\n\n1. prime\n2. prime [unsigned integer]\n";

fn main() {
    let mut args = env::args();
    match args.len() {
        1 => run(None),
        2 => run(args.nth(1)),
        _ => {
            println!("{}", HELP_TEXT);
        }
    }
}

fn run(num_str: Option<String>) {
    match num_str {
        Some(n) => match process_string(&n) {
            Ok(num) => {
                let prime_checker = PrimeChecker::from(num);
                println!("{}", prime_checker);
            }
            Err(e) => println!("An error occurred during execution: \n{}", e),
        },
        None => ask_for_input(),
    }
}

fn ask_for_input() {
    println!("Prime cheker utility.\n=====================\n");
    loop {
        match process_single_line() {
            Ok(prime_checker) => {
                println!("{}", prime_checker);
            }
            Err(e) => match e {
                MyError::InvalidDigit(msg) => {
                    println!("{}\nPlease try again", msg);
                }
                MyError::Other => {
                    println!("An error occured while processing the input.\nPlease try again.")
                }
            },
        }
        if user_wants_to_exit() {
            break;
        }
    }
}

fn user_wants_to_exit() -> bool {
    let mut usr_str = String::new();

    println!("Do you want to exit? (y/n) : ");
    stdin()
        .read_line(&mut usr_str)
        .expect("Error while reading input.");

    let trimmed = usr_str.trim();

    trimmed == "y" || trimmed == "Y" || trimmed.to_lowercase() == "yes"
}

/// Parses a text to string and checks if it's a prime number
/// Fails if the String can't be parsed correctly.
fn process_single_line() -> Result<PrimeChecker, MyError> {
    let mut num_str: String = String::new();
    println!("Enter the number to check : ");
    stdin().read_line(&mut num_str).unwrap();
    let number = process_string(num_str.trim())?;

    Ok(PrimeChecker::from(number))
}

fn process_string(num_str: &str) -> Result<u64, MyError> {
    num_str
        .parse::<u64>()
        .map_err(|_| MyError::InvalidDigit(format!("\"{}\" is not a valid digit.", num_str)))
}

/// This type has a u64 and an indicator `is_prime` that indicates if the number is
/// a prime number or not
struct PrimeChecker {
    integer: u64,
    is_prime: bool,
}

impl From<u64> for PrimeChecker {
    fn from(other: u64) -> PrimeChecker {
        PrimeChecker {
            integer: other,
            is_prime: prime::is_prime(other),
        }
    }
}

impl fmt::Display for PrimeChecker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_prime {
            write!(f, "The integer {} is a prime.", self.integer)
        } else {
            write!(f, "The integer {} is not a prime.", self.integer)
        }
    }
}

#[derive(Debug)]
enum MyError {
    InvalidDigit(String),
    // place your different errors here
    Other,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::InvalidDigit(decription) => write!(f, "{}", decription),
            MyError::Other => write!(f, "Unexpected error"),
        }
    }
}

impl std::error::Error for MyError {}
