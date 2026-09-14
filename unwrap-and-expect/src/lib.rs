
//
// Rust provides two methods for handling unrecoverable errors unwrap and expect, they can be used
// on both Result and Option types, they will trigger a panic! if the value is an Err(E) variant for
// Result<T, E> or a None variant for Option<T>
//
// These methods extract the inner value but terminate the program if the operation fails.
// They are particularly useful for quick prototyping or situations where an error is truly unrecoverable.
//
// In this challenge, you will interact with file operations to demonstrate the use of unwrap and expect.
// Instead of directly working with Result or Option, you will use standard library functions that
// return these types and handle their results using unwrap and expect.
//
// Your Task
// Implement the following functions:
//
// read_file_to_string: This function takes a file path as input, attempts to read its contents,
// and returns the contents as a String. Use expect to handle any file I/O errors with a custom
// error message of exactly Failed to read file: <path>.

// get_env_variable: This function retrieves the value of an environment variable by name.
// Use unwrap to panic if the variable is not set.
// Notes
// expect provides a way to add context to your panics, which can help with debugging.
// unwrap is more concise but less descriptive when errors occur.

pub fn read_file_to_string(path: &str) -> String {
    // 1. Implement the function
    let contents_as_string = std::fs::read_to_string(path).expect(&format!("Failed to read file: {path}!"));
    contents_as_string
}

pub fn get_env_variable(key: &str) -> String {
    // 2. Implement the function
    let db_url = std::env::var(key).unwrap();
    db_url
}

/// Example usage
pub fn main() {
    // Example 1: Using read_file_to_string
    let file_content = read_file_to_string("example.txt");
    println!("File content: {}", file_content);

    // Example 2: Using get_env_variable
    std::env::set_var("EXAMPLE_KEY", "example_value");
    let value = get_env_variable("EXAMPLE_KEY");
    println!("Environment variable value: {}", value);

    // Must panic
    read_file_to_string("nonexistent.txt");
    get_env_variable("MISSING_KEY");
}
