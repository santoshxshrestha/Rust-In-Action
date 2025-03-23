fn main() {
    let number = Some(2542);
    if let Some(i) = number {
        println!("The val is {}", i)
        //here we have to write if let both times to deconstruct the val inside the Some
    } else if let Some(i) = number {
        println!("The val is {}", i)
    }
}
