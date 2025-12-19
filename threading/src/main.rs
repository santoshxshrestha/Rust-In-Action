use std::thread;
use std::time::Duration;
fn main() {
    // stroing the starting time
    let starting_time = std::time::Instant::now();

    let mut join_handles = vec![];
    for i in 0..100 {
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_secs(10));
            println!(
                "my name is thread {} I just worke up!!! from Hibernation ",
                i
            )
        });
        join_handles.push(handle);
    }

    for handle in join_handles {
        let _ = handle.join();
    }
    // recording the elapsed time
    let elapsed = starting_time.elapsed();
    println!("Elapsed time: {:?}", elapsed);
}
