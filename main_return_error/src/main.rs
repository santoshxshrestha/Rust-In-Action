pub fn return_result(left: i32, right: i32) -> Result<i32, Box<dyn std::error::Error>> {
    if left < 0 || right < 0 {
        Err("Negative numbers are not allowed".into())
    } else {
        Ok(left + right)
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let left: i32 = 0;
    let right: i32 = 3;

    let sum = return_result(left, right)?;
    println!("The sum is {}", sum);
    Ok(())
}
