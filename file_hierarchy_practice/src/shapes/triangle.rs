        use crate::drawing::Drawable;

        // Triangle struct with base, height, and sides
        pub struct Triangle {
            pub base: f64,
            pub height: f64,
            pub side_a: f64,
            pub side_b: f64,
            pub side_c: f64,
        }

        impl Triangle {
            // Constructor
            pub fn new(base: f64, height: f64, side_a: f64, side_b: f64, side_c: f64) -> Triangle {
                Triangle { base, height, side_a, side_b, side_c }
            }

            // Calculate the area of the triangle
            pub fn area(&self) -> f64 {
                0.5 * self.base * self.height
            }

            // Calculate the perimeter of the triangle
            pub fn perimeter(&self) -> f64 {
                self.side_a + self.side_b + self.side_c
            }
        }

        // Implement the Drawable trait for Triangle
        impl Drawable for Triangle {
            fn draw(&self) {
                println!("Drawing a triangle with base: {:.2} and height: {:.2}", self.base, self.height);
                println!("  -> Area: {:.2}", self.area());
                println!("  -> Perimeter: {:.2}", self.perimeter());
            }
        }
