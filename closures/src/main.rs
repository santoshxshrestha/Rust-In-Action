#![allow(unused)]
use std::{mem, vec};

//so we can use this this to apply the 
//closure just once and drop the val or transfer the ownership out
//there in the function 
fn apply<F>(f:F)where 
F:FnOnce() {
    f();
}

//these things are called the higher order functions
//because they take function (or closure ) as an arguments 
fn apply_to_4<F>(f:F) -> i32 where
F:Fn(i32) -> i32 {
    f(4)
}


fn main() {
    let a = 3;
    let b = 52;
    // let add = || println!("The sum is {}", a + b);
    // add();
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    println!("The sum is {}", add(a, b));

    {
        let movable = Box::new(253);
        //here i have forcefully moved the val in the closure
        let consume = move || {
            println!("val is moved here is {}", movable);
        };
        consume();
    }

    {
        let movable = Box::new(42);
        let consume = || {
            println!("val is moved here is {}", movable);
            //here the val is dropped
            mem::drop(movable);
        };
        consume();
    }

    {
        let something = vec![42, 523, 423, 52];
        let contains = move |needle| something.contains(needle);
        println!("It contain the val:42 {}", contains(&42));
    }

    {
        let greeting = "hello anish";
        let mut farewell = "goodbye anish".to_owned();

        let diary = || {
            println!("I said {}", greeting);

            farewell.push_str("!!!");
            println!("Then I screamed {}",farewell);
            println!("Now I can sleep. zzz");

            mem::drop(farewell);

        };
        apply(diary);
    }

    //lets apply the same thing but this time we will just apply more 
    //not just once 
    {
        let double = |x| 2*x;
        println!("4 is doubled : {}", apply_to_4(double));

        let triple = |x| 3*x;
        println!("4 is tripled : {}", apply_to_4(triple));

    }
}
