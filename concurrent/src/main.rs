use tokio::time::{Duration, sleep};

async fn download(id: u8) {
    println!("Starting async download {}", id);
    sleep(Duration::from_secs(1)).await;
    println!("Finished async download {}", id);
}

#[tokio::main]
async fn main() {
    let handles = vec![
        tokio::spawn(download(1)),
        tokio::spawn(download(2)),
        tokio::spawn(download(3)),
    ];

    for handle in handles {
        handle.await.unwrap();
    }

    println!("All downloads completed (async)");
}
