use core::fmt;
use std::fmt::Formatter;
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, " ( {} {} )\n ( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn transpose(matrix: Matrix) -> Matrix {
    let Matrix(one, two, three, four) = matrix;
    Matrix(one, three, two, four)
}

fn main() {
    let tuple_of_tuple = ((43i32, 54i32, 23u32, 2.9f32), 1i32, 25i32, 25f32);
    println!("{:?}", tuple_of_tuple.0);
    println!("{}", tuple_of_tuple.0 .3);

    //we can't print more then 12 elements in the tuple
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("Too long tuple: {:?}", too_long_tuple);
    println!("One element tuple: {:?}", (5u32,));
    println!("Just an integer: {}", (5u32));
    println!("Just an integer: {:?}", (5u32));

    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));
}
