#![allow(unused)]
use std::fs::File;
use std::io::prelude::*;
use std::panic;
use std::path::Path;

static LOREM_IPSUM: &str =
    "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

fn main() {
    {
    let path = Path::new("hello.txt");
    let display = path.display();


    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
        
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}",display, s),
        
    }
    //after the file goes out of scope the 
    //"hello.txt" file gets automatically closed
    }

    println!("----------------------------------------------------------------");

    {
        //the following mode will create a file if there is not any and rewrites if 
        //there is not any
        let path = Path::new("write.txt");
        let display = path.display();

        let mut file  = match File::create(&path){
            Err(why) => panic!("countn't create {}: {}",display, why),
            Ok(file)=> file,
        };
// as_bytes is used here becasue the write_all does not allows 'statis string
        match file.write_all(LOREM_IPSUM.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}",display),
        }
    }
}
