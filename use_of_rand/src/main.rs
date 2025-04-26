#![allow(unused)]
use rand::prelude::*;
fn main() {
    let mut rng = rand::rng();
    let random_int : [u8;100] = rng.random();
    println!("The random integer is :{:?}",random_int);

    let mut numbers:Vec<i32> = (1..100).collect();
    let selection = numbers.choose(&mut rng);
    match selection {
        Some(x) => println!("The selected one is {x}."),
        None => println!("Received None")

    }

    println!("random unicode point is:{}",rng.random::<char>());

}
