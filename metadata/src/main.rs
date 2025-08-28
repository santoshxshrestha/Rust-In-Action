use std::path::Path;
fn main() {
    // let medatata = fs::read_to_string("Cargo.toml").expect("Unable to read file");
    let path = Path::metadata(Path::new("./Cargo.toml"));
    if let Ok(metadata) = path {
        println!("the metadata of the Cargo.toml file is:\n{:#?}", metadata);
    } else {
        println!("Unable to read metadata");
    }
    // println!("the metadata of the Cargo.toml file is:\n{}", medatata);
}
