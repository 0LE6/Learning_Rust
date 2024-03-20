use chrono::{NaiveDate, Utc};

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

pub struct Address {
    street: String,
    city: String, 
    postal_code: u32,
    country: String
}


