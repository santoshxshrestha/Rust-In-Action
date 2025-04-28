#![allow(unused)]

use std::ops::{Add, Sub};

struct Vector2D {
    x: f64,
    y: f64,
}

impl Vector2D{
    fn new(x: f64, y: f64)-> Self{
        Vector2D { x, y }
    }
}

impl Add for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: Self) -> Self::Output {
        println!("> Adding ({}, {}) + ({}, {}).",self.x, self.y, rhs.x, rhs.y);
        Vector2D{
            x : self.x + rhs.x,
            y : self.y + rhs.y,
        }
    }
}

impl Sub for Vector2D {
    type Output = Vector2D;

    fn sub(self, rhs: Self) -> Self::Output {
        println!("> Subtracting ({}, {}) - ({}, {}).",self.x, self.y, rhs.x, rhs.y);
        Vector2D{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            
        }
    }
}

impl Add<f64> for Vector2D {
    type Output = Vector2D;
    fn add(self, scalar: f64) -> Vector2D {
        println!("> Adding vector ({}, {}) + scalar {}",self.x, self.y, scalar);
        Vector2D {
            x:self.x+ scalar,
            y: self.y + scalar,
        }
    }
}

impl std::fmt::Display for Vector2D{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}).",self.x, self.y)
    }

}


fn main() {

    let vector1 = Vector2D::new(30.43, 20.5);
    let vector2 = Vector2D::new(42.2,52.42);

    let sum =vector1  + vector2 ;
    println!("The sum is {}",sum);



}
