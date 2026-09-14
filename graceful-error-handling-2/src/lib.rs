//
// Let's improve previous example a little bit by returning a custom error type instead of a plain string.
// This will allow us to define specific error types and provide more structured error handling.
//
// Your Task
// The logic of the function remains the same as the previous challenge, but the returned error type
// is what you need to change.
//
// Define an enum ParsePercentageError with the following variants:
//
// InvalidInput: for inputs that cannot be parsed as numbers.
// OutOfRange: for numbers that are not in the range 0-100.
// Implement the Error trait for ParsePercentageError. Use the std::error::Error trait and provide
// human-readable descriptions for each error.
//
// Update the parse_percentage function to:
//
// Return Ok(u8) if the input is a valid percentage (between 0 and 100).
// Return Err(ParsePercentageError::OutOfRange) if the number is out of range.
// Return Err(ParsePercentageError::InvalidInput) if the input is not a valid number.

// 1. Finish the definition
#[derive(Debug, PartialEq, Eq)]
pub enum ParsePercentageError {
    OutOfRange,
    InvalidInput,
}

impl std::fmt::Display for ParsePercentageError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParsePercentageError::OutOfRange => write!(f, "percentage is out of range"),
            ParsePercentageError::InvalidInput => write!(f, "invalid percentage input"),
        }
    }
}

impl std::error::Error for ParsePercentageError {}

// 2. Implement the `Error` trait

pub fn parse_percentage(input: &str) -> Result<u8, ParsePercentageError> {
    // 3. Implement this function
    let number: u8 = match input.parse() {
        Ok(num) => num,
        Err(_) => return Err(ParsePercentageError::InvalidInput),
    };
    if number > 100 {
        return Err(ParsePercentageError::OutOfRange)
    };
    Ok(number)
}

// Example usage
pub fn main() {
    let result = parse_percentage("50");
    println!("{:?}", result); // Should print: Ok(50)

    let result = parse_percentage("101");
    println!("{:?}", result); // Should print: Err(ParsePercentageError::OutOfRange)

    let result = parse_percentage("abc");
    println!("{:?}", result); // Should print: Err(ParsePercentageError::InvalidInput)
}

