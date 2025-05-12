#![allow(unused)]
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;

//setting the cound of the thread to 3
static NTHREADS: i32 = 3;
 


fn main() {
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
    for _ in 0..NTHREADS{
        ids.push(rx.recv());
    }

    for child in children {
        child.join().expect("oops! the child thread panicked");
    }
    println!("{:?}",ids);

}
