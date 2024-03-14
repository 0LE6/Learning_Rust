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

    pub fn variables3() {
        // https://doc.rust-lang.org/rust-by-example/std/str.html#strings
        let mut string = String::from("heap alloc a variable string");
        // this kind of string is mutable (?)
        //string.push_str(" --- this is a concat --- "); // this is not mutable if the string
        // variable doesn't have a mut
        string.push_str(" --- this is a concat --- ");
        println!("printing a declared variable as a string -> {}", string);
    }
}


fn main() {
    // module practice
    // every module has different exercises 
    //basic_exercises::variables();
    //basic_exercises::variables2();
    basic_exercises::variables3();
}

