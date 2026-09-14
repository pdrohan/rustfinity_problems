// In this challenge, you will implement a function that filters strings from a slice, returning
// only those that start with a given keyword. The function will return an iterator over the
// filtered results. This approach demonstrates how to combine Rust's iterator combinators with
// the impl Trait syntax.

// from https://doc.rust-lang.org/rust-by-example/trait/impl_trait.html
// impl trait can be used as an argument type or as a return type
// if the function is generic over a trait but you don't mind the specific type, you can simplify the
// function declaration using impl Trait as the type of the argument
// fn func (R: std::io;Bufread>(src: R) -> std::io::Result<Vec<Vec<String>>
// which means func is generic allowing any type which implements Bufread, its not important what type
// R is and R is only used to declare the type of src

// in this example I am not sure what the bufread is in this scenario
// these are the hints
// Use .iter() to iterate over references to the strings in the slice.
// The filter method takes a closure to apply a filtering condition.
// Use the starts_with method to check if a string starts with a keyword.
// Use move in the closure to capture the keyword.

// this is for the return position, this is an abstraction that lets us define the return type
// cleanly instead of having to define the compiler generated concrete type

pub fn filter_starts_with<'a>(strings: &'a[String], keyword: &'a str)
                      -> impl Iterator<Item = &'a String> {
    strings.iter().filter(move |&x| x.starts_with(keyword))
}

// ok so the learnings here are 

// Example usage
pub fn main() {
    let input = vec![
        String::from("apple"),
        String::from("apricot"),
        String::from("banana"),
        String::from("cherry"),
    ];
    let filtered: Vec<&String> = filter_starts_with(&input, "ap").collect();
    println!("{:?}", filtered); // Expected output: ["apple", "apricot"]
}
