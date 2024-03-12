pub mod basic_exercises {
    pub fn variables() {
        // practice basic variables in Rust
        let num = 69;
        println!("Printing the integer variable --> {}", num)
    }

    pub fn variables2() {
        let num: i32 = 420;
        println!("printing number with i32 ==> {}", num)
    }
}


fn main() {
    // module practice
    // every module has different exercises 
    //basic_exercises::variables();
    basic_exercises::variables2();
}
