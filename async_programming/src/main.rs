#![allow(unused)]
use tokio::time::Duration;
use tokio::time::sleep;

pub async fn hello() {
    println!("hello world from santosh");
}

pub async fn hello_back() {
    // sleep(std::time::Duration::from_secs(1000)).await;
    sleep(Duration::from_millis(1000)).await;
    println!("Hello santosh from rust");
}

#[tokio::main]
async fn main() {
    //this is lazy function as they are needed to be awaited to make them work
    hello();
    hello_back().await;
}
