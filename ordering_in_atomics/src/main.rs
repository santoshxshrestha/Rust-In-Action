#![allow(unused)]
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
fn main() {
    // {
    //     let atomic = Arc::new(AtomicU64::new(0));
    //     let clone_of_atomic = atomic.clone();
    //     let joinhandle = thread::scope(|s| {
    //         thread::spawn(move || {
    //             clone_of_atomic.store(100, Ordering::Release);
    //         })
    //     });
    //     if let Err(e) = joinhandle.join() {
    //         eprintln!("you got errro here {e:?}");
    //     }
    //
    //     let another_joinhandle = thread::scope(|s| {
    //         thread::spawn(move || {
    //             let val = atomic.store(121, Ordering::Relaxed);
    //             let val_to_be_printed = atomic.load(Ordering::Relaxed);
    //             println!("the value is {val_to_be_printed}");
    //         })
    //     });
    //
    //     if let Err(e) = another_joinhandle.join() {
    //         eprintln!("you got errro here {e:?}");
    //     }
    // }
    let count = Arc::new(AtomicU64::new(0));
    let joinhandle = thread::scope(|s| {
        let clone_of_arc = Arc::clone(&count);
        s.spawn(|| clone_of_arc.fetch_add(1, Ordering::Relaxed));

        s.spawn(|| clone_of_arc.fetch_add(2, Ordering::Relaxed));

        s.spawn(|| {
            println!("the count now is {:?}",);
        })
    });
}
