        use crate::drawing::Drawable;

        // Rectangle struct with width and height
        pub struct Rectangle {
            pub width: f64,
            pub height: f64,
        }

        impl Rectangle {
            // Constructor
            pub fn new(width: f64, height: f64) -> Rectangle {
                Rectangle { width, height }
            }

            // Calculate the area of the rectangle
            pub fn area(&self) -> f64 {
                self.width * self.height
            }

            // Calculate the perimeter of the rectangle
            pub fn perimeter(&self) -> f64 {
                2.0 * (self.width + self.height)
            }
        }

        // Implement the Drawable trait for Rectangle
        impl Drawable for Rectangle {
            fn draw(&self) {
                println!("Drawing a rectangle with width: {:.2} and height: {:.2}", self.width, self.height);
                println!("  -> Area: {:.2}", self.area());
                println!("  -> Perimeter: {:.2}", self.perimeter());
            }
        }

