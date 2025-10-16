use core::fmt;

pub enum KnownShape {
    Circle(f64),
    Rectangle(f64, f64),
    Square(f64),
}
impl KnownShape {
    pub fn area(&self) -> f64 {
        match self {
            KnownShape::Circle(r) => std::f64::consts::PI * r * r,
            KnownShape::Square(l) => l * l,
            KnownShape::Rectangle(l, b) => l * b,
        }
    }
}
impl fmt::Display for KnownShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KnownShape::Circle(_) => write!(f, "The area of the circle is {:.2}", self.area()),
            KnownShape::Rectangle(_, _) => {
                write!(f, "The area of the rectangle is {:.2}", self.area())
            }
            KnownShape::Square(_) => write!(f, "The area of the square is {:.2}", self.area()),
        }
    }
}

fn demo_enum() {
    let shapes = vec![
        KnownShape::Circle(6.32),
        KnownShape::Rectangle(1.4, 4.43),
        KnownShape::Square(4.2),
    ];

    for shape in &shapes {
        println!("{}", shape)
    }
}

fn main() {
    demo_enum();
}
