#![allow(unused)]
use std::error::Error;
use std::string;

use tokio::net::TcpListener;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> std::result::Result<(), std::io::Error> {
    println!("Tcp server is live");
    let listner = TcpListener::bind("127.0.0.1:8080").await?;
    let mut buff = vec![0; 1024];

    loop {
        match listner.accept().await {
            Ok((tcpstream, socketaddress)) => {
                tcpstream.readable().await?;

                match tcpstream.try_read(&mut buff) {
                    Ok(n) => buff.truncate(n),
                    Err(_) => println!("Error:"),
                }
            }
            Err(e) => println!("got the error here {} ", e),
        }
        let readable = String::from_utf8_lossy(&buff[..]);
        println!("Got :{}", readable);
    }
    Ok(())
}

