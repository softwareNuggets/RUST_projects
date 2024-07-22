// Import HashMap from the standard library
use std::collections::HashMap;

fn main() {

    // HashMap  
    // key-value pair storage and retrieval

    let mut grades1 = HashMap::new();
    
    let mut grades2: HashMap::<String, i32> = HashMap::new();
    
    let mut grades3: HashMap::<String, i32> = 
                    HashMap::with_capacity(10);
    
    grades1.insert(String::from("Robin"), 95);
    
    grades2.insert(String::from("Baron"), 90);

    //grades3.insert(String::from("Jason"), 91);
    grades3.insert(String::from("Nicole"), 91);

    println!("grade1.capacity -> : {}", grades1.capacity());
    println!("grade2.capacity -> : {}", grades2.capacity());
    println!("grade3.capacity -> : {}", grades3.capacity());

    println!("grade1.len -> : {}", grades1.len());
    println!("grade2.len -> : {}", grades2.len());
    println!("grade3.len -> : {}", grades3.len());

    // Print values using get method
    println!("Robin's grade: {:?}", grades1.get("Robin"));
    println!("Baron's grade: {:?}", grades2.get("Baron"));
    println!("Jason's grade: {:?}", grades3.get("Jason"));   
    println!("Nicole's grade: {:#?}",grades3.get("Nicole")); //pretty print
}