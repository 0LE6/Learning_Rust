use std::fmt;

struct Coordinates {
    x: i32,
    y: i32,
}

pub fn formatting() {
     
    // to format a struct lol
    impl fmt::Display for Coordinates {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
             write!(f, "({}, {})", self.x, self.y)
        }
    }

    let my_struct = Coordinates { x: 10, y: 20};
    print!("My struct of coordinates -> {}", my_struct);

    /* example with debug

        #[derive(Debug)]
        struct Coordinates {
            x: i32,
            y: i32,
        }

        fn main() {
            let my_struct = Coordinates { x: 10, y: 20 };
            println!("My struct of coordinates -> {:?}", my_struct);
        }

     */

    println!("\nWithout format --> {}", 3.141592);
    println!("With format --> :.2 (two decimals) {:.2}", 3.141592);

    println!("\nFormat: filling with 0 until beaing a 5 digit output --> :05 {:05}", 42);
}

