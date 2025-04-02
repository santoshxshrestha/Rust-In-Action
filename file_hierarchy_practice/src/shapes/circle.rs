        use crate::drawing::Drawable;
        use std::f64::consts::PI;

        // Circle struct with radius
        pub struct Circle {
            pub radius: f64,
        }

        impl Circle {
            // Constructor
            pub fn new(radius: f64) -> Circle {
                Circle { radius }
            }

            // Calculate the area of the circle
            pub fn area(&self) -> f64 {
                PI * self.radius * self.radius
            }

            // Calculate the perimeter (circumference) of the circle
            pub fn perimeter(&self) -> f64 {
                2.0 * PI * self.radius
            }
        }

        // Implement the Drawable trait for Circle
        impl Drawable for Circle {
            fn draw(&self) {
                println!("Drawing a circle with radius: {:.2}", self.radius);
                println!("  -> Area: {:.2}", self.area());
                println!("  -> Perimeter: {:.2}", self.perimeter());
            }
        }
