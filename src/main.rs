pub mod basic_exercises {
    use std::fmt;

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
        let inmutable_string: &str = "this is an inmutable string";
        println!("\ninmutable string --> {}", inmutable_string);
    }

    pub fn variables4() {
        // here we're going to practice the constant, float and boolean variables
        const CONST_EXAMPLE: f64 = 3.14; // pi number const
        print!("PI const --> {}", CONST_EXAMPLE);

        let float_without_type_specification = 69.0; // is a f64
        println!("Our PI const multiple by the float_without_type_specification -> {}", float_without_type_specification * CONST_EXAMPLE);  

        // this gaves error beacause you can't multiply a f64 by f32
        let _float_with_type_specification: f32 = 2.0;
        //println!("\nOur PI const multiply by the float_with_type_specification {}", float_with_type_specification * CONST_EXAMPLE);

        let mut boolean_var: bool = false;
        if CONST_EXAMPLE == 3.14 {
            boolean_var = true;
            println!("Yep, we're in the if statement that confirms that our const is 3.14 -> {}", boolean_var);
        }
        else {
            println!("Nop, our const is not exactly 3.14 -> {}", boolean_var);
        }

    }

    pub fn formatting() {
        struct Coordinates {
            x: i32,
            y: i32,
        }
        
        // to format a struct lol
        impl fmt::Display for Coordinates {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                 write!(f, "({}, {})", self.x, self.y)
            }
        }

        let my_struct = Coordinates { x: 10, y: 20};
        print!("My struct of coordinates -> {}", my_struct);
    }
}


fn main() {
    // module practice
    // every module has different exercises 
    //basic_exercises::variables();
    //basic_exercises::variables2();
    //basic_exercises::variables3();
    //basic_exercises::variables4();
    basic_exercises::formatting();
}

