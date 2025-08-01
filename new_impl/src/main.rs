#![allow(dead_code)]

use std::fmt::Display;
#[derive(Debug)]
pub struct Rectangle {
    lenght: i32,
    breadth: i32,
}
impl Rectangle {
    fn new(lenght: i32, breadth: i32) -> Rectangle {
        Self { lenght, breadth }
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "the lenght of rectangle is {} and breadth is {}.",
            self.lenght, self.breadth,
        )
    }
}

fn main() {
    let varaible = Rectangle::new(12, 32);
    println!("{}", varaible);

    println!(
        "content {}",
        format_args!("this is format args {} here is the vla ", 3)
    );
}
