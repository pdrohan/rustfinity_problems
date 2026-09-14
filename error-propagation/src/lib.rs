// Error propagation is a core concept in Rust that allows you to handle errors in a clean and
// structured way. Instead of having to handle each error manually on every step,
// you can easily use the ? operator to propagate errors to a higher level so that they can
// be handled in a single place.
//
// In this challenge, you’ll use io::Error to represent potential issues when working with file I/O.
// This approach leverages Rust’s standard library for concise and idiomatic error handling.
//
// Your task is to implement a function that reads integers from a file, computes their sum,
// and gracefully propagates any errors using the ? operator.
//
// Implement the function sum_integers_from_file:
//
// Takes the file path as a parameter.
// Reads the file line by line, assuming each line contains a single integer or invalid data.
// Computes and returns the sum of all integers as a Result<i32, io::Error>.
// Handles the following:
// If the file cannot be opened, propagate the io::Error.
// If a line cannot be parsed as an integer, propagate a custom io::Error with a meaningful message.
// Requirements
// Handle errors cleanly and propagate them using the ? operator.
// For invalid lines, create an io::Error with io::ErrorKind::InvalidData.

use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::BufRead;

pub fn sum_integers_from_file(file_path: &str) -> Result<i32, io::Error> {
    // TODO: Implement this function
    // Hint: Use `File::open`, `BufReader::new`, and `.lines()` to process the file.
    // Use `?` to propagate errors and `io::Error::new` for custom errors.
    // if succeeded, give me the value inside Ok, else immediately return an error from this function
    // without ?, open returns Result<File, io::Error>
    let mut f = File::open(file_path)?;
    let reader = BufReader::new(f);
    let mut lines = reader.lines();

    let mut sum: i32 = 0;

    for i in lines {
        let st_num = match i {
            Ok(i) => i,
            Err(error) => return Err(error)
        };

        let number: i32 = match st_num.parse() {
            Ok(number) => number,
            Err(error) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    error,
                ));
            }
        };

        sum += number;
    }

    Ok(sum)

}

// Example usage
pub fn main() {
    let file_path = "numbers.txt";

    match sum_integers_from_file(file_path) {
        Ok(sum) => println!("The sum is: {}", sum),
        Err(e) => eprintln!("Error: {}", e),
    }
}
