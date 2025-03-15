#![allow(dead_code)]
use std::convert::From;
use std::convert::Into;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

#[derive(Debug)]
struct Another {
    value: i64,
}
impl Into<Another> for i64 {
    fn into(self) -> Another {
        Another { value: self }
    }
}
fn main() {
    let num1 = Number::from(30);
    println!("My number is {:?}", num1);

    let num2 = 5i64;
    let num: Another = num2.into();
    println!("My number is {:?}", num);
}
