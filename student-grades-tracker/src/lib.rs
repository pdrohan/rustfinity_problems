use std::collections::HashMap;



// Now that we know about vectors and hashmaps, let’s tackle something more challenging.
// In this exercise, we’ll build a system to manage and analyze student grades.
// This challenge will help you practice working with structs, hashmaps, vectors, and methods while managing a collection of data.
//
// The focus here is on functionality. Ignore error handling for this challenge; assume the input is always valid.
//
// Your Task
// You need to define the StudentGrades struct, which contains a HashMap of student names (String)
// as keys and Student structs as values. Each Student struct should have the following fields:
//
// name: The name of the student (String).
// grades: A Vec<u8> to store the student's grades.
// Implement the following methods for the StudentGrades struct:
//
// add_grade(name: &str, grade: u8): Add a grade to an existing student.
// get_grades(name: &str) -> &[u8]: Retrieve the grades of a student as an immutable reference.
// Since there’s no error handling, assume all inputs are valid, and keys will exist when accessed.
// Hints
// Click here to reveal hints
// Use the HashMap methods like entry, insert, and get to manage student data.


pub struct Student {
    name: String,
    grades: Vec<u8>
}

pub struct StudentGrades {
    pub students: HashMap<String, Student>,
}

impl StudentGrades {
    pub fn new() -> Self {
        Self {
            students: HashMap::new(),
        }
    }

    // 3. Implement the methods
    pub fn add_student(&mut self, name: &str) {
        // add_student(name: &str): Add a new student to the HashMap. If the student already exists, do nothing.
        let contains_name: bool = self.students.contains_key(name);

        if !contains_name {
            let student_to_add: Student = Student{name: name.to_string(), grades: Vec::new()};
            self.students.insert(name.to_string(), student_to_add);
        }
    }

    pub fn add_grade(&mut self, name: &str, grade: u8) {
        // add_grade(name: &str, grade: u8): Add a grade to an existing student.
        if let Some(student) = self.students.get_mut(&name.to_string()) {
            student.grades.push(grade);
        }

    }

    pub fn get_grades(&self, name: &str) -> &[u8] {
        // Implement here
        self.students.get(name).unwrap().grades.as_slice()
    }
}

// Example usage
pub fn main() {
    let mut tracker = StudentGrades::new();

    tracker.add_student("Alice");
    tracker.add_student("Bob");

    tracker.add_grade("Alice", 85);
    tracker.add_grade("Alice", 90);
    tracker.add_grade("Bob", 78);

    println!("{:?}", tracker.get_grades("Alice")); // [85, 90]
    println!("{:?}", tracker.get_grades("Bob")); // [78]
}
