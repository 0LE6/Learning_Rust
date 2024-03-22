use chrono::{NaiveDate, Utc};

// https://doc.rust-lang.org/book/ch17-01-what-is-oo.html

// pseudo class Student
pub struct Student {
    name: String, 
    surname: String, 
    birth_date: NaiveDate, // equivalent to DateTime in C#
    grade: f32,
    email: String,
    is_active: bool,
    credits_earned: u32,
    start_date: NaiveDate,
    address: Address,
    phone_number: Option<String>
}

// pseudo class Address
pub struct Address {
    street: String
    city: String, 
    postal_code: u32,
    country: String
}


// using Default trait 
impl Default for Student {
    fn default() -> Self {
        Self {
            name: String::new(),
            surname: String::new(),
            birth_date: NaiveDate::from_ymd(1900,1,1),
            grade: 0.0,
            email: String::new(),
            is_active: false,
            credits_earned: 0,
            start_date: Utc::today().naive_utc(),
            address: Address::defauñt(), // TODO : add constructor fro the Address struct
            phone_number: None,
        }
    }
}



