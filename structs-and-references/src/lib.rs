// 1. Define the struct
pub struct TextFinder<'a> {
    reffed: &'a str
}

impl<'a> TextFinder<'a> {

    pub fn new(str_slice: &'a str) -> TextFinder<'a> {
        Self {
            reffed: str_slice
        }
    }

    pub fn find_first(&self, keyword: &'a str) -> Option<&'a str> {
        // given the referenced value is a multiline string i need some function to split by line then
        // find within the substring i guess ? so we use split and find
        let lines: Vec<&str> = self.reffed.split('\n').collect();

        for line in lines {
            match line.find(keyword) {
                Some(_index) => { return Some(line) },
                None => continue
            }
        }
        None
    }

    pub fn find_many(&self, keyword: &'a str) -> Vec<&'a str> {
        // same same but construct the valid lines vec instead of returning hte first instance
        let lines: Vec<&str> = self.reffed.split('\n').collect();

        let mut valid_lines: Vec<&'a str> = Vec::new();

        for line in lines {
            match line.find(keyword) {
                Some(_index) => { valid_lines.push(line) },
                None => continue
            }
        }
        valid_lines

    }
}

// 2. Implement the struct and define the methods

// Example usage
pub fn main() {
    let text = "Rust is fast and memory-efficient.\nOwnership is key to Rust's safety.\nRustaceans love the borrow checker.";
    let finder = TextFinder::new(text);

    let first = finder.find_first("Rust");
    println!("{:?}", first); // Should print: Some("Rust is fast and memory-efficient.")

    let matches = finder.find_many("Rust");
    println!("{:?}", matches); // Should print: ["Rust is fast and memory-efficient.", "Ownership is key to Rust's safety."]
}
