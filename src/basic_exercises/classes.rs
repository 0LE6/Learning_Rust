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
    street: String,
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
            birth_date: NaiveDate::from_ymd_opt(1900, 1, 1).expect("Invalid date"),
            grade: 0.0,
            email: String::new(),
            is_active: false,
            credits_earned: 0,
            start_date: Utc::now().naive_utc().date(),
            address: Address::default(),
            phone_number: None,
        }
    }
}

impl Student {
    // Getters
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_surname(&self) -> &str {
        &self.surname
    }

    pub fn get_birth_date(&self) -> &NaiveDate {
        &self.birth_date
    }

    pub fn get_grade(&self) -> f32 {
        self.grade
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn get_credits_earned(&self) -> u32 {
        self.credits_earned
    }

    pub fn get_start_date(&self) -> &NaiveDate {
        &self.start_date
    }

    pub fn get_address(&self) -> &Address {
        &self.address
    }

    pub fn get_phone_number(&self) -> Option<&String> {
        self.phone_number.as_ref()
    }

    // Setters
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_surname(&mut self, surname: String) {
        self.surname = surname;
    }

    pub fn set_birth_date(&mut self, birth_date: NaiveDate) {
        self.birth_date = birth_date;
    }

    pub fn set_grade(&mut self, grade: f32) {
        self.grade = grade;
    }

    pub fn set_email(&mut self, email: String) {
        self.email = email;
    }

    pub fn set_is_active(&mut self, is_active: bool) {
        self.is_active = is_active;
    }

    pub fn set_credits_earned(&mut self, credits_earned: u32) {
        self.credits_earned = credits_earned;
    }

    pub fn set_start_date(&mut self, start_date: NaiveDate) {
        self.start_date = start_date;
    }

    pub fn set_address(&mut self, address: Address) {
        self.address = address;
    }

    pub fn set_phone_number(&mut self, phone_number: Option<String>) {
        self.phone_number = phone_number;
    }

    pub fn to_string(&self) -> String {
        format!(
            "Name: {}, Surname: {}, Birth Date: {}, Grade: {}, 
            Email: {}, Active: {}, Credits: {}, Start Date: {}, 
            Address: {}, Phone: {:?}",
            self.name,
            self.surname,
            self.birth_date,
            self.grade,
            self.email,
            self.is_active,
            self.credits_earned,
            self.start_date,
            self.address.to_string(),
            self.phone_number
        )
    }
}

use std::fmt;

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}


impl Default for Address {
    fn default() -> Self {
        Self {
            street: String::new(),
            city: String::new(),
            postal_code: 0,
            country: String::new(),
        }
    }
}

impl Address {
    // Constructor
    pub fn new(street: String, city: String, postal_code: u32, country: String) -> Self {
        Self {
            street,
            city,
            postal_code,
            country,
        }
    }

    // Getters
    pub fn get_street(&self) -> &str {
        &self.street
    }

    pub fn get_city(&self) -> &str {
        &self.city
    }

    pub fn get_postal_code(&self) -> u32 {
        self.postal_code
    }

    pub fn get_country(&self) -> &str {
        &self.country
    }

    // Setters
    pub fn set_street(&mut self, street: String) {
        self.street = street;
    }

    pub fn set_city(&mut self, city: String) {
        self.city = city;
    }

    pub fn set_postal_code(&mut self, postal_code: u32) {
        self.postal_code = postal_code;
    }

    pub fn set_country(&mut self, country: String) {
        self.country = country;
    }

    pub fn to_string(&self) -> String {
        format!(
            "{} {}, {} {}",
            self.street, self.city, self.postal_code, self.country
        )
    }
}


