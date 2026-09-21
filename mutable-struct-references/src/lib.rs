// 1. Finish the struct definition
// read as MutableTextFinder contains a mutable reference to a str, and that reference has lifetime 'a.
pub struct MutableTextFinder<'a> {
    // finder has a lifetime parameter named 'a
    // the reference stored in the struct is valid for 'a
    txt: &'a mut String
}

// 2. Implement the methods for the struct
// introduce lifetime param 'a so i can use it in the impl block
// this impl is for finder whos ref has lifetime 'a
impl <'a> MutableTextFinder<'a> {

    // new accepts a reference with lifetime 'a and returns text finder containing the same lifetime
    // -> this is where it passed  along
    pub fn new(txt: &'a mut String) -> MutableTextFinder<'a> {
        Self {txt,}
    }

    pub fn find_first(&self, keyword: &str) -> Option<&str> {
        let lines: Vec<&str> = self.txt.split('\n').collect();

        for line in lines {
            match line.contains(keyword) {
                true => return Some(line),
                false => continue
            }
        }
        None
    }

    pub fn replace_lines(&mut self, keyword: &str, replacement_str: &str) {
        let updated: Vec<&str> = self
            .txt
            .lines()
            .map(|line| {
                if line.contains(keyword) {
                    replacement_str
                } else {
                    line
                }
            })
            .collect();

        // Reconstruct the string with original newline separation
        *self.txt = updated.join("\n");
    }


    pub fn get_text(&'a self) -> &'a str {
        self.txt
    }

}

// Example usage
pub fn main() {
    let mut text = String::from("Rust is awesome\nLearning Rust\nFun with Rustaceans");
    let mut finder = MutableTextFinder::new(&mut text);

    let first = finder.find_first("Rust");
    println!("{:?}", first); // Should print: Some("Rust is awesome")

    finder.replace_lines("Rust", "Programming in Rust");
    println!("{}", finder.get_text()); // Should print the modified text
}
