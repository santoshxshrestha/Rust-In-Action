use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    print!("Enter the first number: ");
    io::stdout().flush().expect("Failed to flush output");
    io::stdin().read_line(&mut input)
        .expect("Failed to read input");

    let num1: i32 = input.trim().parse()
        .expect("Invalid number!");


    let mut input = String::new();

    print!("Enter the second number: ");
    io::stdout().flush().expect("Failed to flush output");
    io::stdin().read_line(&mut input)
        .expect("Failed to read input");

    let num2: i32 = input.trim().parse()
        .expect("Invalid number!");

    // Calculation 
    let sum: i32 = num1 + num2;

    println!("The sum of the two numbers {} and {} is {}", num1, num2, sum);
    
}

