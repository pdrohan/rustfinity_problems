// this too me 45 mins with some other tasks mixed in

pub fn get_first_element(numbers: Vec<i32>, min_value: i32) -> Result<i32, String> {
    // Finish the function
    let first_element = numbers.first(); // <- Returns an Option<&i32>
    // ok_or returns a result and raises the error if not ok
    let result = first_element.ok_or("Vector is empty");

    // I think this is verbose, but it does make sense, was jumping through hoops on the fact that
    // the references were not lining up , since the function expects i32 not the reference to it
    let val = match result {
        Ok(result) => *result,
        Err(_) => return Err("Vector is empty".to_string())
    };

    if val < min_value {
        return Err("First element is below the minimum allowed value".to_string())
    };
    Ok(val)
}

// Example usage
pub fn main() {
    let numbers = vec![10, 20, 30, 40, 50];

    match get_first_element(numbers.clone(), 15) {
        Ok(value) => println!("First valid value: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    let empty_numbers: Vec<i32> = vec![];
    match get_first_element(empty_numbers, 15) {
        Ok(value) => println!("First valid value: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}
