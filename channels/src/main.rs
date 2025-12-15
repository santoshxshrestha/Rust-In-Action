#![allow(unused)]
use std::sync::mpsc::{self, Receiver, Sender, SyncSender, sync_channel};
use std::thread;

///setting the count of the thread to 3
static NTHREADS: i32 = 3;

fn main() {
    /// This block is for asynchronous channel using mpsc
    {
        /// In summary, SyncSender is a version of Sender that uses synchronization to ensure that the sender blocks if the receiver isn't ready to consume messages,
        /// giving you fine control over communication flow between threads.
        let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
        let mut children = Vec::new();

        for id in 0..NTHREADS {
            /// This is how name was set like mpsc here we are clonning the  tx to thread_tx (multi producer)
            let thread_tx = tx.clone();

            let child = thread::spawn(move || {
                ///here we are sending to rx and if we get any errors then the program will panic
                thread_tx.send(id).unwrap();
                println!("thread {} finished", id);
            });

            children.push(child);
        }

        // This reserves space for NTHREADS elements in the vector to avoid multiple allocations
        let mut ids = Vec::with_capacity(NTHREADS as usize);

        /// Here we are receiving the values from the threads and pushing them to the vector ids
        /// this will block the main thread until it receives a message from each of the spawned threads
        /// but it don't care about the order of the messages as well as the other content like the
        /// print thinge after the send so we are joining after the receiving part
        for _ in 0..NTHREADS {
            ids.push(rx.recv());
        }

        /// making sure that main thread will not exit before the child threads are executed like the
        /// print thinge happens after the message sending
        for child in children {
            child.join().expect("oops! the child thread panicked");
        }
        println!("{:?}", ids);
    }
    ///  this block is for synchronous channel using mpsc
    {
        let (tx, rx) = sync_channel(3); // Channel with capacity 3
        for _ in 0..3 {
            let tx = tx.clone(); // Clone the sender for each thread
            thread::spawn(move || tx.send("ok").unwrap()); // Each thread sends "ok"
        }
        drop(tx); // Drop the last sender to signal the receiver that no more messages will be sent
        while let Ok(msg) = rx.recv() {
            // Receive messages until all senders are dropped
            println!("{msg}");
        }
        println!("completed");
    }
}
