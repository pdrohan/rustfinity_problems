use std::fs::File;
use std::io::Read;
pub fn read_file(file_path: &str) -> Option<String> {
    // TODO: Implement this function
    // .ok() converts Result -> Option
    // File::open(file_path).ok() converts Ok(file) -> Some(file), Err(error) -> None
    // ? unwraps Option so if the opening fails, it returns none
    let mut f = File::open(file_path).ok()?;
    let mut contents = String::new();
    // so f is actually a file atp, but it also returns a result, so tag .ok and ?
    // fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize>
    f.read_to_string(&mut contents).ok()?;
    return Some(contents);
}

// Example usage
pub fn main() {
    let file_path = "example.txt";

    match read_file(file_path) {
        Some(contents) => println!("File contents:\n{}", contents),
        None => println!("Failed to read the file."),
    }
}
