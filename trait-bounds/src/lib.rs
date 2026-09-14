// TODO: Define the generic function `compare_and_display` with appropriate trait bounds.

//Define a generic function compare_and_display that:
//
// Takes two parameters of the same type.
// Returns the greater of the two using the PartialOrd trait.
// Use trait bounds to ensure the function works only with types that implement both Display and PartialOrd.
pub fn compare_and_display<T: std::fmt::Display + PartialOrd>(a:T, b:T) -> T{
     if a > b {
          a
     } else {
          b
     }
}

// Example usage
pub fn main() {
     let greater = compare_and_display(10, 20);
     println!("Greater value: {}", greater);

     let greater = compare_and_display("Apple", "Orange");
     println!("Greater value: {}", greater);
}
