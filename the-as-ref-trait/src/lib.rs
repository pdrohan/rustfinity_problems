pub fn print_message<T: AsRef<str>>(s: T) {
    println!("{}", s.as_ref())
}

// Example usage
pub fn main() {
    // Example 1: Using a &str
    print_message("Hello, world!");

    // Example 2: Using a String
    let greeting = String::from("Welcome to Rust!");
    print_message(greeting);
}
