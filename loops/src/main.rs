#![allow(unreachable_code, unused_labels)]

fn main() {
    // let mut count = 0;
    // loop {
    //     count += 1;
    //     println!("hello santosh");
    //     if count != 10 {
    //         continue;
    //     }
    //     break;
    // }
    // // here 'outer and 'inner is just loop lables
    // 'outer: loop {
    //     println!("Entered the outer loop");
    //
    //     'inner: loop {
    //         println!("Entered the inner loop");
    //
    //         // This would break only the inner loop
    //         //break;
    //
    //         // This breaks the outer loop
    //         break 'outer;
    //     }
    //
    //     println!("This point will never be reached");
    // }
    //
    // println!("Exited the outer loop");
    let key: u64 = 100000082;
    let mut pass: u64 = 0;
    loop {
        pass += 1;
        if pass == key {
            println!("{pass}");
            break println!("The key was {pass}");
        }
    }
}
