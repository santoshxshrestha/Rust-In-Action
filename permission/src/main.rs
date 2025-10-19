use std::fs::metadata;
use unix_perms::display_permissions;

fn main() -> std::io::Result<()> {
    let meta = metadata("Cargo.toml")?;
    let mode_str = display_permissions(&meta);
    println!("{}", mode_str);
    Ok(())
}
