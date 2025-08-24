#![allow(unused)]
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

//setting the count of the thread to 3
static NTHREADS: i32 = 3;

fn main() {
    // In summary, SyncSender is a version of Sender that uses synchronization to ensure that the sender blocks if the receiver isn't ready to consume messages,
    // giving you fine control over communication flow between threads.
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let mut children = Vec::new();

    for id in 0..NTHREADS {
        let thread_tx = tx.clone();

        let child = thread::spawn(move || {
            //here we are sending to rx and if we get any errors then the program will panic
            thread_tx.send(id).unwrap();
            println!("thread {} finished", id);
        });

        children.push(child);
    }

    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        ids.push(rx.recv());
    }

    for child in children {
        child.join().expect("oops! the child thread panicked");
    }
    println!("{:?}", ids);
}
