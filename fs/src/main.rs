use std::fs;
use std::io::Write;
fn main() {
    let file_path = "example.txt";
    let mut f = fs::File::create(&file_path).unwrap();
    f.write_all(b"this is another content out there").unwrap();
}
