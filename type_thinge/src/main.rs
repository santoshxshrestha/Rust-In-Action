#![allow(unused)]
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

// Define a type alias for a thread-safe, shared message
type Message = Arc<Mutex<String>>;

pub fn talk(msd: Message) {
    thread::scope(|s| {
        s.spawn(|| {
            let mut msd = msd.lock().unwrap();
            msd.push_str("\nHello from another part of the cpu");
        });
        s.spawn(|| {
            let mut msd = msd.lock().unwrap();
            msd.push_str("\nHello from yet another part of the cpu");
        });
    })
}

fn main() {
    let data = Arc::new(Mutex::new(String::from("Hello, world!")));
    let data_clone = Arc::clone(&data);

    let handle = thread::spawn(move || {
        let mut data = data_clone.lock().unwrap();
        data.push_str("\nHello from some part of the cpu");
    });

    handle.join().unwrap();
    talk(Arc::clone(&data));

    println!("{}", data.lock().unwrap());
}
