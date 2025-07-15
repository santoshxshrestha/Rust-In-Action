#![allow(unused)]
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let name = Arc::new("Santosh");

    for i in 1..=10 {
        let name = Arc::clone(&name);

        thread::spawn(move || {
            println!(
                "The iteration number is {} and the pinting val is {}",
                i, name
            );
        });
        //here new threads are spawned in the gap of the 1sec and have
        //no any impact with the printing the thigns
        thread::sleep(Duration::from_secs(1));
    }
}
