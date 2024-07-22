// Import HashMap from the standard library
use std::collections::HashMap;

fn main() {

    // UPDATING a value in a HashMap

    
    // HashMap  
    // key-value pair storage and retrieval
    let mut grades1: HashMap::<String, i32> = HashMap::new();
    
    grades1.insert(String::from("Robin"), 95);
    
     // Print values using get method
    println!("Robin's grade: {:?}", grades1.get("Robin"));

    
    // Check if "Robin" exists in grades1
    if let Some(&original_value) = grades1.get("Robin") {
	
        // Update Robin's grade
        grades1.insert(String::from("Robin"), 98);
        
        println!("Original value was {}; updated value is {}", 
                                 original_value, grades1["Robin"]);
								 
    } else {
        println!("Robin not found in grades1");
    }
   
}
