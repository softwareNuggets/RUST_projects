fn ex1_using_range()

	let num:i32 = 5;
    ex2_use_break(num);
  
  ex3_iterate_in_reverse();
  
  ex4_use_end_points();
  
  ex5_step_by();
  
  ex6_step_by_in_reverse();
  
  ex7_use_continue();
  
  ex8_use_filter();
  
  ex9_use_enumerate();
  
  
fn ex1_using_range() {

    //The half-open range start..end 
    //is used to create a range that includes 
    //the start value but 
    //excludes the end value. 
    
    for i in 0..10 {
        println!("i = {}",i);
    }
}




fn ex2_use_break(index:i32) {

    for i in 0..10 {
        println!("i = {}",i);
        
        if i==index {
            println!("exit loop at {}",index);
            break;
        }
        
    }
}

fn ex3_iterate_in_reverse() {

    //The half-open range start..end 
    
    for i in (0..10).rev() {
    
        println!("i = {}",i);
        
    }
    
}

fn ex4_use_end_points() {

    println!("use end points");
    
    for i in 6..=10 {
        println!("i = {}",i);
    }
}


fn ex5_step_by() {

    println!("use step_by...increment by 2");
    
    for i in (6..=10).step_by(2) {
        println!("i = {}",i);
    }
}




fn ex6_step_by_in_reverse() {

    println!("use step_by...increment by 2");
    
    for i in (6..=10).rev().step_by(2) {
        println!("i = {}",i);
    }
}


fn ex7_use_continue() {

    println!("use continue keyword if odd");
    
    for i in 1..10 {
    
        if i%2==1 {
            continue;
        }
        
        println!("i = {}",i);
    }
}

fn ex8_use_filter() {

    for i in (0..=10).filter(|&x| x % 2 == 0) {
        println!("Even number: {}", i);
    }    
}


fn ex9_use_enumerate() {
    
    for (index, value) in (0..=20).step_by(2).enumerate() {
        println!("Index: {}, Value: {}", index, value);
    }
    
}