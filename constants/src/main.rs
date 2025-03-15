#![allow(static_mut_refs)]
static LANGUAGE: &str = "English";
static mut COUNT: i32 = 23;
const YEAR: i32 = 3;
const TIME: i32 = 14;

fn main() {
    println!("Hello, world!");
    println!("{}", YEAR);
    println!("{}", TIME);
    println!("{}", LANGUAGE);
    unsafe {
        COUNT += 1;
        println!("The changed vale of the COUNT is: {}", COUNT);
    }

    println!("{}", LANGUAGE);
}
