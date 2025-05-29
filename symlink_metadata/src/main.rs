use std::fs::symlink_metadata;
fn main() -> std::io::Result<()> {
    let attr = symlink_metadata(".");
    println!("the symlink metadata is {:?}", attr);
    Ok(())
}
