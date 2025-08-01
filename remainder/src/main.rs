pub fn mod_finder(a: i32, b: i32) -> i32 {
    return a % b;
}
fn main() {
    let a = -13;
    let b = 11;
    let remainder = mod_finder(a, b);
    println!("the remainder is {remainder}");
}
