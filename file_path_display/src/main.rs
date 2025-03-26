use std::path::Path;
fn main() {
    let path = Path::new("~/.local/scripts/ cdier");
    println!("The name of the file is {}",path.display());
}
