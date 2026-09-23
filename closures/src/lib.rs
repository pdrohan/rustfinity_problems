


pub fn create_closures() -> (
    impl Fn(i32, i32) -> i32,
    impl Fn(i32, i32) -> i32,
    impl Fn(i32, i32) -> i32,
) {
    let add_closure = |a, b| {
        // Step 1: Implement here
        let addish: i32 = a + b;
        addish
    };

    let subtract_closure = |a, b| {
        // Step 1: Implement here
        let sub: i32 = a - b;
        sub
    };

    let multiply_closure = |a, b| {
        // Step 1: Implement here
        let mult: i32 = a * b;
        mult
    };

    // Step 3:
    // Create the `multiply_closure` closure that multiplies two `i32` values.

    (add_closure, subtract_closure, multiply_closure)
}

// Example usage
pub fn main() {
    let (add, subtract, multiply) = create_closures();

    // Example tests
    assert_eq!(add(3, 4), 7); // Expected: 7
    assert_eq!(subtract(10, 4), 6); // Expected: 6
    assert_eq!(multiply(3, 5), 15); // Expected: 15
}
