#![allow(unused)]
use std::any::Any;
use std::boxed::Box;

pub trait Shapes {
    fn name(&self) -> String;
    fn area(&self) -> f64;
    fn as_any(&self) -> &dyn Any;
}

pub struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Self {
        Self { radius: radius }
    }
    fn circumference(&self) -> f6 {
        2.0 * std::f64::consts::PI * self.radius
    }
    fn message(&self) -> String {
        format!("Hello there my name is {}", self.name())
    }
}

impl Shapes for Circle {
    fn name(&self) -> String {
        String::from("circle")
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Square {
    length: f64,
}

impl Square {
    fn new(length: f64) -> Self {
        Self { length: length }
    }

    fn perimeter(&self) -> f64 {
        4.0 * self.length
    }

    fn message(&self) -> String {
        format!("Hello there my name is {}", self.name())
    }
}

impl Shapes for Square {
    fn name(&self) -> String {
        String::from("Square")
    }

    fn area(&self) -> f64 {
        self.length * self.length
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Rectangle {
    length: f64,
    breadth: f64,
}

impl Rectangle {
    fn new(lenght: f64, breadht: f64) -> Self {
        Self {
            length: lenght,
            breadth: breadht,
        }
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.length + self.breadth)
    }

    fn message(&self) -> String {
        format!("Hello there my name is {}", self.name())
    }
}

impl Shapes for Rectangle {
    fn name(&self) -> String {
        String::from("Rectangle")
    }

    fn area(&self) -> f64 {
        self.length * self.breadth
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn with_out_downcasting() {
    let trait_object: Vec<Box<dyn Shapes>> = vec![
        Box::new(Circle::new(50.23)),
        Box::new(Square::new(91.23)),
        Box::new(Rectangle::new(312.13, 31.21)),
    ];

    for object in trait_object {
        println!("The name of the object is {}", object.name());
        println!("The area of the object is {:.2}", object.area());
    }
}

fn main() {
    with_out_downcasting();
}
