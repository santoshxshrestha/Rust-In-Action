use std::{thread, time::Duration};

fn main() {
    println!("Main thread: Starting the program");

    let background_thread = thread::spawn(|| {
        println!("Background thread: I'm Starting work");

        for i in 1..=5 {
            println!("Background thread: working... step {}", i );
            thread::sleep(Duration::from_secs(5)); //her we are passing small pause
        }

        println!("Background thread: Work complete!");
    });

    println!("Main thread: Going to sleep for 1 second");


    //when the main thread goes to sleep the background_thread starts to work
    thread::sleep(Duration::from_secs(1));
    println!("Main thread: woke up after 1 second");

    println!("Main thread : Doing some other work while background thread continues");

    println!("Main thread: waithing for background thread to finish");

    //this line will let the background_thread to run and if there are some 
    //potential error then the unwrap will handel it 
    background_thread.join().unwrap();

    println!("Main thread: All done!");
 

    println!("---------------------------------------------------------------------");

    {
        let mut children = vec![];
        for i in 0..10{
            //here the thread is sored in the vector
            children.push(thread::spawn(move || {
                println!("this is the spawned thread of number {}",i);
            }));
        }
        for child in children {
            let _ = child.join().expect("could not join the thread");
        }

    }
}
