#![allow(unused)]
pub struct Vehicle {
    name: String,
    no_of_wheels: u8,
    price: u32,
}

pub trait Car {
    fn name(name: String) -> String;
}

impl Car for Vehicle {
    fn name(name: String) -> String {
        name
    }
}

fn main() {
    let now = Vehicle::name("this is name".to_string());
    println!("{}", now);
}
