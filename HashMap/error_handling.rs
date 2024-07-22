use std::collections::HashMap;
use std::error::Error;
use std::fmt;

// Custom error type
#[derive(Debug)]
struct GradeError {
    message: String,
}

impl fmt::Display for GradeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for GradeError {}

// Function to get a grade, returning a Result
fn get_grade(grades: &HashMap<String, i32>, student: &str) -> Result<i32, GradeError> {
    grades.get(student)
          .copied()
          .ok_or_else(|| GradeError { message: format!("No grade found for student: {}", student) })
}

// Function to update a grade, returning a Result
fn update_grade(grades: &mut HashMap<String, i32>, student: &str, new_grade: i32) -> Result<(), GradeError> {
    if new_grade < 0 || new_grade > 100 {
        return Err(GradeError { message: format!("Invalid grade: {}. Grade must be between 0 and 100", new_grade) });
    }
    
    grades.insert(String::from(student), new_grade);
    Ok(())
}

fn main() {
    let mut grades = HashMap::new();
    grades.insert(String::from("Alice"), 95);
    grades.insert(String::from("Bob"), 80);

    // Example 1: Successfully get a grade
    match get_grade(&grades, "Alice") {
        Ok(grade) => println!("Alice's grade: {}", grade),
        Err(e) => println!("Error: {}", e),
    }

    // Example 2: Try to get a non-existent grade
    match get_grade(&grades, "Charlie") {
        Ok(grade) => println!("Charlie's grade: {}", grade),
        Err(e) => println!("Error: {}", e),
    }

    // Example 3: Successfully update a grade
    match update_grade(&mut grades, "Bob", 85) {
        Ok(()) => println!("Successfully updated Bob's grade"),
        Err(e) => println!("Error: {}", e),
    }

    // Example 4: Try to update with an invalid grade
    match update_grade(&mut grades, "Alice", 120) {
        Ok(()) => println!("Successfully updated Alice's grade"),
        Err(e) => println!("Error: {}", e),
    }

    // Print final grades
    println!("Final grades: {:?}", grades);
}