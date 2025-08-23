#![allow(unused)]
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use thread::Thread;

fn main() {
    {
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
    // here i am going to do some muts in the arc
    {
        let arc = Arc::new(Mutex::new(String::from(
            "this is some heap allocated string",
        )));
        for i in 1..=10 {
            let clone_of_arc = Arc::clone(&arc);
            thread::spawn(move || match clone_of_arc.lock() {
                Ok(content) => println!("the content is {}", content),
                Err(e) => eprintln!("you got some error {e}"),
            });
        }
    }
    // this session was really brain breaking but finally got it
}
