#![allow(dead_code)]
use std::fmt;

struct Vector {
    args: Vec<String>,
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.args.join(" "))
    }
}

fn main() {
    let args = Vector {
        args: std::env::args().collect(),
    };
    println!("The env thing caputred are\n :{} ", args);
}
