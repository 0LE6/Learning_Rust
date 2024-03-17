pub mod basic_exercises;

fn main() {
    // module practice
    // every module has different exercises 
    
    //basic_exercises::variables();
    //basic_exercises::variables2();
    //basic_exercises::variables3();
    //basic_exercises::variables4();

    //basic_exercises::formatting::formatting();
    //basic_exercises::variables::variables4();
    
    let mut vector = basic_exercises::vectors::vectors();
    println!("My initial Vector created in vectors.rs --> {:?}", vector);
    
    vector.push(0);
    println!("Pushing a 0 to our vector --> {:?}", vector);

    vector.push(69);
    println!("Pushing a 69 to our vector --> {:?}", vector);

    if let Some(last) = vector.pop() {
        println!("Pushing a 0 to our vector --> {:?}", vector);
        println!("The last element to pop was --> {}", last);
    } // removing the last value and showing it


}
