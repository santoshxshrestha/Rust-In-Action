fn main() {

    println!("1 + 2 = {}", 1u32 + 2);
    //this line doesn't works but the line above will work becaue of the rust infers the 2 as u32
    //above
    // println!("1 + 2 = {}", 1u32 + 2i32);
    println!("1 + 2 = {}", 1i32 + 2);
    // the scientific notation will be printed in following way
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);
    println!("One million is written as {}", 1_000_000u32);
}
