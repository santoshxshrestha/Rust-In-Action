use tokio::{
    spawn,
    time::{Duration, sleep},
};

async fn say_hello() {
    sleep(Duration::from_millis(100)).await;
    println!("hello");
}

async fn say_world() {
    sleep(Duration::from_millis(100)).await;
    println!("world");
}

#[tokio::main]
async fn main() /*-> Result<(), Box<dyn std::error::Error>> */
{
    // so if we do this then they will not be parallal any more
    // spawn(say_hello()).await?;
    // spawn(say_world()).await?;
    // Ok(())

    let say_hello = spawn(say_hello());
    let say_world = spawn(say_world());

    // we can do this to make them parral
    let _ = say_world.await;
    let _ = say_hello.await;
}
