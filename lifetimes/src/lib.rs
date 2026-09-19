pub fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    // this is about the lifetimes, when working with references we need lifetimes to know at compile time
    // to prevent dangling references w/o the garbage collector
    let s1len: usize = s1.chars().count();
    // use chars.count because there is a test with emojis and we need to measure by unicode scalar
    let s2len: usize = s2.chars().count();
    if s1len > s2len {
        s1
    } else if s2len > s1len {
        s2
    } else {
        s1
    }
}

// Example usage
pub fn main() {
    let s1 = "short";
    let s2 = "longer string";

    let result = longest(s1, s2);
    println!("The longest string is: {}", result);
    assert_eq!(result, "longer string");

    let s3 = "equal";
    let s4 = "equal";
    let result = longest(s3, s4);
    println!("The longest string is: {}", result);
    assert_eq!(result, "equal");
}
