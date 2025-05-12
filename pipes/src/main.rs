#![allow(unused)]
use std::io::prelude::*;
use std::process::{self, Command, Stdio};
use std::thread::panicking;

static PANGRAM: &'static str = "the quick brown fox jumps over the lazy dog\n";

fn main() {
    let mut cmd = if cfg!(target_family = "windows") {
        let mut cmd = Command::new("powershell");
        cmd.arg("-Command").arg("$input | Measure-Object -Line -Word -Character");
        cmd
    }else {
        Command::new("wc")
    };
    let  process = match cmd
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn(){
            Err(why) =>panic!("coundn't spawn wc: {}",why),
            Ok(process)=> process,
    };


    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()){
        Err(why) => panic!("couldn't write to wc stdin: {}",why),
        Ok(_) => println!("send pangram to wc"),
    }

    let mut s = String::new();
    match  process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {}",why),
        Ok(_) => println!("wc responded with: \n{}",s),
        
    }
}
