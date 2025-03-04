// Only types that implement fmt::Display can be formatted with `{}`. User-
// defined types do not implement fmt::Display by default.
//
//this is most
//https://doc.rust-lang.org/std/fmt/

//for getting the type
use std::any::type_name;

//we say this helper function
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    println!("Here is the two: {}", 2);
    //here we got new line
    //
    print!("here is no new line.");

    println!(
        "My name is {1}, and his name is {0}, I am {age} years old and he is {age} years old",
        "Asutosh",
        "Santosh",
        age = 20
    );
    println!("Base 10:               {}", 121212);
    println!("Base 2 (binary):       {:b}", 121212);
    println!("Base 8 (octal):        {:o}", 121212);
    println!("Base 16 (hexadecimal): {:x}", 121212);

    {
        println!("{number:>5}", number = 1);
        // padding of  white spaces 5 where the 5th will be the number self

        println!("{number:0>5}", number = 1);
        //padding of 0 where the 5th will be the number self

        println!("{number:0>5}", number = 10);
        //I can use it in some project related to time

        println!("{:05}", 10);
        //this is the short hand for it

        println!("{number:0<5}", number = 10);
        //padding frome right
        println!("{number:0<5}", number = 11);
    }

    {
        // You can use named arguments in the format specifier by appending a `$`.
        println!("{number:0>width$}", number = 11, width = 5);
    }

    {
        let num: i32 = 11;
        let width: usize = 4;
        println!("{num:0>width$}")
    }

    {
        //format won't print any thing
        let format = format!("{1} {} {0} {}", 1, 2); // => "2 1 1 2"
        print!("{format}\n");
    }

    {
        let pi = 3.141592;
        println!("pi is roughly {pi:.3}");
        //here e is the format specifier for scientific notation
        println!("{pi:e}");
    }

    {
        let argument = 2 + 2;
        let some_thing = format!("{argument}"); // => "4"
        print!("{some_thing}\n");

        fn make_string(a: u32, b: &str) -> String {
            format!("{a } {b}")
        }
        let string = make_string(927, "label"); // => "label 927"
        print!("{string}\n");
    }
    {
        let a: i32 = 60;
        let b: &i32 = &a;
        println!("{b:p}");
        //here p is the format specifier for pointer
    }

    {
        //using the inference here
        let x = "santosh";
        let some = type_of(x);
        println!("{some}");
    }
    {
        //remember that the type inference also has limitation
        let y: u64 = 13491649131956916;
        let typy = type_of(y);
        println!("{typy}");
    }
    {
        println!("Here the val of the x is :{:>1$}", "santosh", 20);
        //you can see the difference here
        println!("Here the val of the x is :{:1$}!", "santosh", 20);
    }
}
