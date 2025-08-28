use std::fs;
fn main() {
    if let Err(e) = fs::create_dir("hello") {
        eprintln!("could not create dir : {e}");
    }
}
