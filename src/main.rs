pub mod basic_exercises;
use crate::basic_exercises::classes::Address;

fn main() {
    // module practice
    // every module has different exercises 
    
    //basic_exercises::variables();
    //basic_exercises::variables2();
    //basic_exercises::variables3();
    //basic_exercises::variables4();

    //basic_exercises::formatting::formatting();
    //basic_exercises::variables::variables4();
   
    /*
    let mut vector = basic_exercises::vectors::vectors();
    println!("My initial Vector created in vectors.rs --> {:?}", vector);
    
    vector.push(0);
    println!("Pushing a 0 to our vector --> {:?}", vector);

    vector.push(69);
    println!("Pushing a 69 to our vector --> {:?}", vector);

    if let Some(last) = vector.pop() {
        println!("Pushing a 0 to our vector --> {:?}", vector);
        println!("The last element to pop was --> {}", last);
    } // removing the last value and showing it with .pop()

    .contains()
    if vector.contains(&69) {
        print!("The vector contains the 69.");
    }
    else {
        print!("The vector does not contain the 69 already.");  
    }

    vector.insert(1,69);
    print!("\nUsing .insert(1,69) to insert the 69 in the position 1 --> {:?}", vector);

    // .len() and .clear()
    if vector.len() >= 3 {
        vector.clear();
        print!("\nClearing the vectro using .clear() if vector is >= 3 --> {:?}", vector);
    }
     

    let my_array = basic_exercises::arrays::arrays();

    print!("The num in the index 0 --> {}", my_array[0]); // 1
    print!("\nThe num in the index 3 --> {}", my_array[3]); // 3
    
    // now with iteration
    for &num in my_array.iter() {
        print!("\nNumber --> {}", num);
    }

    */
    
    let mut student = basic_exercises::classes::Student::default();
    student.set_name("Rust".to_string());
    student.set_surname("Lang".to_string());

    let address = Address::new(
        "123 Fake St".to_string(), 
        "Anytown".to_string(),
        69,
        "Rustyland".to_string());
    
    student.set_address(address);

    println!("{}", student);
    
    // continue from here ... :)
    // according to more professional sources of info than gpt
    // the setting and getting in rust is different, not Java-ish, some links for info:
    //  https://www.reddit.com/r/rust/comments/d7w6n7/is_it_idiomatic_to_write_setters_and_getters/
    // f*ck, I need to continue with this... don't have time

    
    // for loop
    for i in 0..5 {
        print!("\nCount --> {}", i);
    }
}
