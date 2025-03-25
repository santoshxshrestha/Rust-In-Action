#![allow(unused)]
use std::{mem, vec};
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
}
