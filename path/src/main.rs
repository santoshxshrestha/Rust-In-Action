#![allow(unused)]
use std::path::{self, Path};

fn main() {
    let path = Path::new("/home/santosh/dev/Rust-In-Action");

    let display = path.display();
    println!("{display}");

    println!();
    //while joining the path there should not be '/'  directory included again
    let mut new_path = path.join("path/").join("src/");
    println!("after joining the path and src {}",new_path.display());

    println!();
    new_path.push("main.rs");
    println!("after pushing the main.rs{}",new_path.display());

    println!();
    new_path.set_file_name("lib.rs");
    println!("after setting the file name to the lib.rs{}",new_path.display());

    println!();

    match new_path.to_str() {
        None => panic!("latest path is not a valid UTF-8 sequence"),
        Some(s) => println!("latest path is {}",s),
        
    }
}
