#![allow(unused)]
use std::sync::Arc;
use std::sync::atomic::{AtomicI64, Ordering};
use std::thread;
fn main() {
    let atomic_type = Arc::new(AtomicI64::new(0));
    let data = String::from("this is santosh");

    // let scope = thread::scope(|s| {
    //     s.spawn(|| println!("this is the message from the thread scope:{}", data));
    //
    //     s.spawn(|| println!("this is another message from the thread scope:{}", data));
    // });

    // let clone_of_data = Arc::clone(&atomic_type);
    // let handle = thread::spawn(move || println!("the count is {:?}", clone_of_data));
    // handle.join();
    //
    // let handle = (1..=10).map(|_| {
    //     let clone_of_data = Arc::clone(&atomic_type);
    //     thread::spawn(move || clone_of_data.fetch_add(1, Ordering::SeqCst))
    // });
    //
    // for item in handle {
    //     if let Err(e) = item.join() {
    //         eprintln!("failed to join the handle:{:?}", e);
    //     }
    // }

    // move || {
    //     println!("hello santosh the number is: {}", data);
    // };
    //

    let thread_scope = thread::scope(|s| {
        (1..=10).map(|_| {
            let clone_of_data = Arc::clone(&atomic_type);
            s.spawn(|| {})
        })
    });
    println!(
        "the latest count is {}",
        atomic_type.load(Ordering::Acquire)
    );
}
