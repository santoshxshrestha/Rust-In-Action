#[derive(Debug)]
pub enum Version {
    FirstVersion,
    SecondVersion,
}

pub fn parse_version(version: &[u8]) -> Result<Version, &'static str> {
    match version.get(0) {
        Some(1) => Ok(Version::FirstVersion),
        Some(2) => Ok(Version::SecondVersion),
        None => Err("invalid version"),
        _ => Err("invalid version selection"),
    }
}

fn main() {
    let app = [2, 1, 2, 3];
    // if let Ok(t) = parse_version(&app) {
    //     println!("version : {:?}", t)
    // }
    match parse_version(&app) {
        Ok(t) => println!("version: {:?}", t),
        Err(e) => println!("error: {}", e),
    }
}
