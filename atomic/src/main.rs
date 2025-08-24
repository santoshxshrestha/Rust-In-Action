#![allow(unused_imports)]
use std::mem::transmute;
use std::sync::Arc;
use std::sync::atomic;
use std::sync::atomic::AtomicI8;
use std::sync::atomic::AtomicI16;
use std::sync::atomic::Ordering;
use std::thread;
fn main() {
    let arc = Arc::new(atomic::AtomicBool::new(true));
    thread::scope(|s| {
        s.spawn(|| {
            arc.store(false, Ordering::Relaxed);
            println!(
                "the content of the arc now is :{}",
                arc.load(Ordering::Relaxed)
            );
        });
    });

    thread::scope(|scope| {
        scope.spawn(|| {
            println!(
                "the content of the arc in another thread is :{}",
                arc.load(Ordering::Relaxed)
            );
        });
    });
    println!(
        "this is the main thread and the content here is :{}",
        arc.load(Ordering::Relaxed)
    );
    {
        let atomic_arc: Arc<atomic::AtomicI8> = Arc::new(atomic::AtomicI8::new(127));
        thread::scope(|scope| {
            scope.spawn(|| unsafe {
                let different_type = transmute::<&AtomicI8, &AtomicI16>(&atomic_arc);
                println!(
                    "the content in the different type :{}",
                    different_type.load(Ordering::Relaxed)
                );
                different_type.store(255, Ordering::Relaxed);
                println!(
                    "again the content  after over writing {}",
                    different_type.load(Ordering::Relaxed)
                );
            });
        })
    }
}
