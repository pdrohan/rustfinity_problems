// In Rust, traits define shared behavior that types can implement. A trait acts as a contract,
// requiring implementing types to provide certain methods. This makes traits one of the most
// powerful features for achieving polymorphism and code reusability in Rust.
//
// For example, the Display trait allows custom formatting for types using the {} formatter.
// Similarly, you can define your own traits to describe behaviors relevant to your program.
//
// Your Task
// In this challenge, you'll implement a trait Describable and use it to define a common interface
// for objects that can provide a description. Your task is to:
//
// Define a trait Describable with a single method describe that returns a String.
// Implement this trait for the struct Person.
// Implement this trait for the struct Book.
// Requirements
// The Person struct should have fields name: String and age: u8.
// The Book struct should have fields title: String and author: String.
// The describe method for Person should return a string like "Person: Alice, Age: 30".
// The describe method for Book should return a string like "Book: Rust Programming, Author: Jane Doe".

pub trait Describable {
    fn describe(&self) -> String;
}

pub struct Person {
    pub name: String,
    pub age: u8,
}

impl Describable for Person {
    fn describe(&self) -> String {
        format!("Person: {}, Age: {}", self.name, self.age)
    }
}

pub struct Book {
    pub title: String,
    pub author: String,
}

impl Describable for Book {
    fn describe(&self) -> String {
        format!("Book: {}, Author: {}", self.title, self.author)
    }
}


// Example usage
pub fn main() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    let book = Book {
        title: "Rust Programming".to_string(),
        author: "Jane Doe".to_string(),
    };

    println!("{}", person.describe());
    println!("{}", book.describe());
}
