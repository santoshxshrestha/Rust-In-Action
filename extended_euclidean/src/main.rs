#![allow(unused, non_snake_case)]
use std::io;

fn input() -> i32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failded to read input");
     input.trim().parse::<i32>().expect("Invalid number")
}
fn main() {
    let mut A: i32 = 0;
    let mut B: i32 = 0;
    let mut Q: i32 = 0;
    let mut R: i32 = 0;
    let mut s: i32 = 0;
    let mut s1: i32 = 1;
    let mut s2: i32 = 0;
    let mut t: i32 = 0;
    let mut t1: i32 = 0;
    let mut t2: i32 = 1;
    print!("Enter the first number: \n");

    let mut a: i32 = input();

    print!("Enter the second number: \n");

    let mut b: i32 = input();
    
    if(a>b) {
        A= a;
        B = b;
    }else {
        A = b;
        B = a;
    }

    while (B!=0) {
        Q = A/B;
        R = A%B;
        A = B;
        B = R;
        s = s1 - (Q * s2);
        s1 = s2;
        s2 = s;
        t = t1 - (Q * t2);
        t1 = t2;
        t2 = t;
    }
    println!("The gcd of val is {}",A);
    println!("The value of s = {}, and t = {} wich will satisfy the equeation gcd(a,b) = sa + tb.",s1, s2);

}
