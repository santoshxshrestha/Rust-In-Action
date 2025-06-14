#![allow(unused)]
use std::clone;
use std::{fmt, ptr::slice_from_raw_parts};

struct Student {
    name: String,
    class: u8,
    rollno: u8,
    address: String,
}
impl Student {
    fn new(name: String, class: u8, rollno: u8, address: String) -> Student {
        Student {
            name: name,
            class: class,
            rollno: rollno,
            address: address,
        }
    }
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Hello there my name is {} . I reed in class {}, and my roll number is {}. I live in {}",
            self.name, self.class, self.rollno, self.address
        )
    }
}
impl clone::Clone for Student {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            class: self.class,
            rollno: self.rollno,
            address: self.address.clone(),
        }
    }
}

fn main() {
    let jason = Student::new("Json".to_string(), 14, 89, "Butwal".to_string());
    let hello = jason.clone();
    println!("{}", jason);
    println!("{}", hello);
}
