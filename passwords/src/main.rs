use rpassword;
use std::io::Cursor;
fn main() {
    let password = rpassword::prompt_password("Your password: ").unwrap();
    println!("Your password is {}", password);
}
