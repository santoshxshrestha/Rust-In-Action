    pub trait Drawable {
        fn draw(&self);
    }

    // Function to draw a single shape
    pub fn draw_shape(shape: &impl Drawable) {
        shape.draw();
    }

    // Function to draw multiple shapes from a slice
    pub fn draw_multiple(shapes: &[&dyn Drawable]) {
        for (i, shape) in shapes.iter().enumerate() {
            println!("Shape {}:", i + 1);
            shape.draw();
            println!("----------------------------");
        }
    }
