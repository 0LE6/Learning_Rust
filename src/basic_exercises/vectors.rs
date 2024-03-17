pub fn vectors() -> Vec<i32> {
    // in Rust, vectors of same type can increase and decrease during the execution

    // creating a mutable vector
    let mut vec: Vec<i32> = Vec::new();
    vec.push(4);
    vec.push(2);

    return vec;
    
}
