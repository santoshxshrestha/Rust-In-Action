#![allow(unused)]
use tokio;

#[tokio::main]
async fn main() {
    // you can create following async closure
    let square = async |number: i32| return number * number;

    let number = 2;
    let square_of_number = square(number);
    println!(
        "The square of number {} is {}",
        number,
        square_of_number.await
    );
}
