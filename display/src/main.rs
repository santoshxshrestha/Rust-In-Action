use std::fmt;

fn main() {
    struct Structure(i32);

    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    println!("print the struct by the following way with out the use of derive debug thing:");
    println!("The newly created structure is  is {}", Structure(54));
    let later = Structure(52);
    println!("The value created in the sturcture later is {}", later);
    println!("The value created in the sturcture later is {}", later.0);
    {
        #[derive(Debug)]
        struct MinMax(i64, i64);

        impl fmt::Display for MinMax {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "({}, {})", self.0, self.1)
            }
        }

        #[derive(Debug)]
        struct Point2D {
            x: f64,
            y: f64,
        }

        impl fmt::Display for Point2D {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "x:{}, y: {}", self.x, self.y)
            }
        }
        let minmax = MinMax(5, 62);
        println!("Comparing the sturcutre");
        println!("Display : {}", minmax);
        println!("Debug: {:?}", minmax);

        let big_range = MinMax(-525, 515);
        let small_range = MinMax(-5, 5);
        println!(
            "The big range is {big} and the small is {small}",
            small = small_range,
            big = big_range
        );

        let point = Point2D { x: 5.5, y: 52.5 };

        println!("Compare points:");
        println!("Display: {}", point);
        println!("Debug: {:?}", point);
    }
}
