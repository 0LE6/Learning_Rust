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
            birth_date: NaiveDate::from_ymd_opt(1900,1,1),
            grade: 0.0,
            email: String::new(),
            is_active: false,
            credits_earned: 0,
            start_date: Utc::now().naive_utc().date(),
            address: Address::new(String::new(), String::new(), 0, String::new()),
            phone_number: None,
        }
    }
}

impl Student {
    // Implementa el método to_string() aquí, no dentro de Default
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

    // Añade aquí los getters y setters
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

    pub fn to_string(&self) -> String {
        format!(
            "{} {}, {} {}",
            self.street, self.city, self.postal_code, self.country
        )
    }
}


