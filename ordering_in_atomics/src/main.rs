#![allow(unused)]
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

fn main() {
    let data = Arc::new(AtomicBool::new(false));
    let data_clone = Arc::clone(&data);
    thread::spawn(move || println!("the data is {:?}", data_clone.)).join();
}
