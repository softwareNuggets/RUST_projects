use std::collections::HashMap;

// Function to remove a student from the grades HashMap
fn remove_student(grades: &mut HashMap<String, i32>, student: &str) {
    // Check if the student exists in the HashMap
    if grades.contains_key(student) {
        // If the student exists, remove them and print a confirmation
        grades.remove(student);
        println!("{} has been removed from the grades.", student);
    } else {
        // If the student doesn't exist, print an error message
        println!("{} was not found in the grades.", student);
    }
}

fn main() {
    // Create a new, empty HashMap
    let mut grades1 = HashMap::new();
    
    // Load 4 key-value pairs into grades1
    // Each key is a student's name (String), and each value is their grade (i32)
    grades1.insert(String::from("Robin"), 95);
    grades1.insert(String::from("Baron"), 88);
    grades1.insert(String::from("Jason"), 92);
    grades1.insert(String::from("Nicole"), 97);

    // Print the initial state of the HashMap
    println!("Initial grades:");
    for (student, grade) in &grades1 {
        println!("{}: {}", student, grade);
    }

    println!("\nAttempting to remove students:");

    // Try to remove an existing student
    // &mut grades1: Pass a mutable reference to allow modification
    // "Baron": The key to remove (exists in the HashMap)
    remove_student(&mut grades1, "Baron");
    

    // Try to remove a non-existent student
    // "Pedro": A key that doesn't exist in the HashMap
    remove_student(&mut grades1, "Pedro");
    

    // Print all remaining key-value pairs
    println!("\nRemaining grades after removal attempts:");
    for (student, grade) in &grades1 {
        println!("{}: {}", student, grade);
    }
}
