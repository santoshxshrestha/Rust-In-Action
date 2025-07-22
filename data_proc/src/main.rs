use std::thread;
use std::time::Duration;
use tokio;

pub async fn thread_testing() -> thread::JoinHandle<()> {
    //here we are spawing a thread and doning the work in that thread so we use the closure
    thread::spawn(|| {
        for i in 1..4 {
            println!("hi number {i} from the spawned thread");
            thread::sleep(Duration::from_secs(1));
        }
    })
}

#[tokio::main]
async fn main() {
    println!("This is the first content of the main function");

    //we are calling the thread_testing function which is async fn
    let handle = thread_testing().await;

    if let Err(e) = handle.join() {
        eprintln!("you got some error joining the handle{e:?}")
    }
    println!("this is the content after the thread_testing function is joined");
}
