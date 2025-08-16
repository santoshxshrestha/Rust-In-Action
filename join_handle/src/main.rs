#![allow(unused)]
use std::io;

use tokio::{join, try_join};
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

async fn return_hello() -> Result<String, io::Error> {
    sleep(Duration::from_millis(100)).await;
    Ok("hello".to_string())
}

async fn return_world() -> Result<String, io::Error> {
    sleep(Duration::from_millis(100)).await;
    Ok("world".to_string())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // so if we do this then they will not be parallal any more
    // spawn(say_hello()).await?;
    // spawn(say_world()).await?;
    // Ok(())
    // {
    //     let say_hello = spawn(say_hello());
    //     let say_world = spawn(say_world());
    //
    //     // we can do this to make them parral
    //     let _ = say_world.await;
    //     let _ = say_hello.await;
    // }
    {
        // let (say_hello, say_world) = join!(return_hello(), return_world());
        let (say_hello, say_world) = try_join!(return_hello(), return_world())?;
        println!("{say_hello}");
        println!("{say_world}");
    }
    Ok(())
}
