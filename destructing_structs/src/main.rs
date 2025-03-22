fn main() {
    struct Foo {
        x: (i32, i32),
        y: (i32, i32),
        z: i32,
    }

    let foo = Foo {
        x: (1, 2),
        y: (3, 4),
        z: (5),
    };
    match foo {
        Foo {
            y: (c, b),
            x: (a, 2),
            z: 5,
            //here order is not important in the process of the desturctiong the struct but the
            //order of the values in the struct is important
        } => println!(
            "The vals that are destructured are a: {}, b: {} and c: {}. ",
            a, b, c
        ),

        _ => println!("You got an error while destructuring "),
    }

    {
        struct Bar {
            a: (i32, i32, i32),
            b: (i32, i32),
            c: i32,
        }
        let bar = Bar {
            a: (321, 23, 42),
            b: (323, 42),
            c: (532),
        };
        match bar {
            Bar {
                // In Rust, when destructuring a struct inside a match statement, the variable names you assign inside the pattern will hold the values from the struct. However, if you want to print the field name itself (like a) instead of its value, Rust does not provide a built-in way to do this dynamically because field names are not accessible at runtime.
                b: (c, b),
                a: (a, ..),
                //I cant access the vals in the .. just skipping it
                c: 532,
                //here order is not important in the process of the desturctiong the struct but the
                //order of the values in the struct is important
            } => println!(
                "The vals that are destructured are a: {:?}, b: {} and c: {}. ",
                a, b, c
            ),

            _ => println!("You got an error while destructuring "),
        }
    }

    {
        //destructuring sturcts with out match
        struct Faa {
            a: (i32, i32, i32),
            b: (i32, i32),
            c: i32,
        }

        let faa = Faa {
            a: (3234, 42, 523),
            b: (42, 532),
            c: 532,
        };
        println!(
            "The vals before desturctiong are a: {:?}, b: {:?} and c: {:?}. ",
            faa.a, faa.b, faa.c
        );

        let Faa { a: (a, b, c), .. } = faa;
        println!("The destructured vals are a: {}, b: {} and c: {}", a, b, c)
    }
}
