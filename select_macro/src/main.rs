#![allow(unused)]
use tokio::select;
use tokio::task::{AbortHandle, JoinHandle};
use tokio::time::{Duration, sleep};
use tokio::{self, spawn};

pub async fn say_hello() -> String {
    sleep(Duration::from_secs(5)).await;
    return "saying hello".to_string();
}

pub async fn say_hi() -> String {
    sleep(Duration::from_secs(1)).await;
    return "saying hi".to_string();
}

pub async fn to_be_aborted() -> String {
    sleep(Duration::from_secs(1)).await;
    return "thank  god  I was not abortedy 😅".to_string();
}

#[tokio::main]
async fn main() {
    // ChatGPT said:
    // Exactly — that’s how tokio::select! works:
    // All futures inside the select! start being polled concurrently.
    // The first future that completes returns its value and executes its branch.
    // All other futures are canceled automatically.
    select! {
        // here hello is the vaiable that i going  to store the output of hte say_hello function
        // and the the function body is going to be called
        hello = say_hello() => {
            println!("hello is first: {}",hello)
        }
        hi = say_hi()=> {
            println!("hi is first: {}", hi)
        }
        // here in this  case the say_hello func has the sleep time of 5  sec so the say_hi func
        // will
        // run and teh say_hello will be  aborted we  can do it  manually by  the use of the .abort
        // method
    }

    {
        println!("we  are not  aborting you ");
        let var_for_aborted = to_be_aborted().await;
        println!("called  to_be_aborted func: {}", var_for_aborted);
    }
    {
        println!("we  are  aborting you  here ");
        let joinhandle = spawn(to_be_aborted());
        // we could also  use  the handles by the  use of the vector and then iter throught it
        // and use abort method for each and every item on iteration
        println!("calling  the handle  {:?}", joinhandle.abort());
    }
}
