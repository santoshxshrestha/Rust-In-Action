#![allow(unused)]
use std::fmt::format;

use tokio;

fn name(what: String) -> impl std::future::Future<Output = String> {
    println!("waht is you name ");

    async move {
        println!("this is async call of you name {}", what);
        format!("got your name{}", what)
    }
}

async fn data() -> String {
    println!("return function called");
    "12".to_string()
}

#[tokio::main]
pub async fn main() {
    let s1 = {
        let a = 21;
        format!("The number is {}", a)
    };
    println!("{s1}");

    let s2 = async {
        let b = data().await;
        format!("the reversed number is {}", b)
    };

    println!("before calling s2");
    println!("{}", s2.await);

    let your_name = "Santosh".to_string();
    let future = name(your_name);
    println!("the future is yet to be awaited");
    let name = future.await;
    println!("this is after awaiting the future");
    println!("your content is : {}", name);
}
