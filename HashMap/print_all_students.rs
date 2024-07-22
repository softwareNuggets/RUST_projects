use std::collections::HashMap;

fn main() {

    let mut grades1 = HashMap::new();
    
    // Load 5 key-value pairs into grades1
    grades1.insert(String::from("Robin"), 95);
    grades1.insert(String::from("Charlie"), 88);
    grades1.insert(String::from("Taylor"), 92);
    grades1.insert(String::from("Alex"), 97);
    grades1.insert(String::from("Jordan"), 85);

    // Loop over all entries and print key-value pairs
    println!("All grades:");
    
    for (student, grade) in &grades1 {
        println!("{}: {}", student, grade);
    }
   
}