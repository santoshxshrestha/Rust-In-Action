use std::thread;
use std::time::Duration;
use tokio;

pub async fn thread_testing() {
    //here we are spawing a thread and doning the work in that thread so we use the closure
    let handle = thread::spawn(|| {
        for i in 1..100 {
            println!("hi number {i} from the spawned thread");
            thread::sleep(Duration::from_secs(1));
        }
    });
    handle.join().unwrap();
}

#[tokio::main]
async fn main() {
    println!("This is the first content of the main function");

    //we are calling the thread_testing function which is async fn
    thread_testing().await;
}
