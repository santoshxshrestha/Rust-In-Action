#![allow(unused)]
#[derive(Debug)]
pub enum Name {
    FirstName,
    LastName,
}

pub struct Person {
    pub first_name: Name,
    pub last_name: Name,
}

impl Person {
    pub fn new(first_name: Name, last_name: Name) -> Self {
        Self {
            first_name,
            last_name,
        }
    }
}

fn main() {
    let person = Person::new(Name::FirstName, Name::LastName);
}
