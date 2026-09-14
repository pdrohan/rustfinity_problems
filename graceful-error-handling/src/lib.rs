

// Example usage
// Your task
// In this challenge, you're given a function, parse_percentage that takes a string as input and returns a Result type.
// The function should parse the input string as a percentage and return the percentage as a u8 if the input is valid.
// If the input is invalid, the function should return an error message as a String.
//
// Parsing from a string to a number can fail for many reasons.
// For example, the input string may not be a valid number, or it may be a valid number but
// not a valid percentage. Your task is to handle these errors gracefully and return an error message that explains what went wrong.
//
// Complete the function, if the parsing was successful return a success variant of the Result,
// if there was an error in parsing, return an error variant of the Result with an error message.
//
// Requirements
// If the parse was successful, the function should return the success variant of Result with the percentage as a Ok(u8).
// If the number was out of range (not between 0 and 100), the function should return the error with String Err("Percentage out of range").
// If the string was not a valid number, the function should return the error with String Err("Invalid input").

pub fn parse_percentage(input: &str) -> Result<u8, String> {
    let number: u8 = match input.parse() {
        Ok(num) => num,
        Err(_) => return Err("Invalid input".to_string()),
    };
    if number > 100 {
        return Err("Percentage out of range".to_string());
    }

    Ok(number)
}

pub fn main() {
    let result = parse_percentage("50");
    assert_eq!(result, Ok(50));

    let result = parse_percentage("101");
    assert_eq!(result, Err("Percentage out of range".to_string()));

    let result = parse_percentage("abc");
    assert_eq!(result, Err("Invalid input".to_string()));
}
