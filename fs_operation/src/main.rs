#![allow(unused)]
use std::{fs::{self, File, OpenOptions}, io::{self, Read, Write}, os::unix, path, str::MatchIndices};
use std::path::Path;

use syn::token::PathSep;

fn cat(path: &Path) -> io::Result<String>{
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
        
    }
}

//this is the prototpye of the echo "santsoh" > something.txt where 
//the file does not exists
fn echo(s: &str, path: &Path)-> io::Result<()>{
    let mut f = File::create(path)?;

    f.write_all(s.as_bytes())
}


fn touch(path: &Path) -> io::Result<()>{
    match OpenOptions::new().create(true).write(true).open(path){
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn main() {
    let contents = cat(&Path::new("something.txt"));

    match contents {
        Err(why) => println!("!{:?}",why.kind()),
        Ok(l) => println!("The content of the file are {}",l),
    }

    let str_content:&str = "Hello this is Santosh";
    //this will create a directory and the b.txt as file name 
    echo(str_content, &Path::new("a/b.txt"));


    //this will just create a directory named a
    touch(&Path::new("a/c/b"));

    println!("Making directory");
    match fs::create_dir("a") {
        Err(why) => println!("Error while making directory a {:?}",why.kind()),
        Ok(_) => println!("created new directory named 'a' "),
    }
    println!("");
    println!("");
    println!("");
    println!("creating sym link ln-s ../b.txt a/c/b.txt");
    #[cfg(target_family = "unix")] {
        unix::fs::symlink("../b.txt", "a/c/b.txt").unwrap_or_else(|why| {
            println!(" got error {:?}",why.to_string());
        })
    }

    println!("ls a");
    match fs::read_dir("a") {
        Err(why) => println!("got error {:?}",why.kind()),
        Ok(paths) => for path in paths{
            println!(">{:?}",path.unwrap().path())
        }
        
    }

    println!("rm a/b.txt");
    fs::remove_dir("empty_dir").unwrap_or_else(|why| {
        println!("got error {:?}",why.kind());
    });

    println!("cleaning the directory");
    fs::remove_dir_all("a").unwrap_or_else(|why|{
        println!("got error {:?}",why.kind());
    });


    println!("cleaning the files");
    fs::remove_file("hello.txt").unwrap_or_else(|why| {
        println!("got error {:?}",why.kind());
    });


    println!("cleaning the files");
    fs::remove_file("something.txt").unwrap_or_else(|why| {
        println!("got error {:?}",why.kind());
    })



}
 
