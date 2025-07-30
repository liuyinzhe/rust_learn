use std::{sync::atomic::AtomicUsize, thread, time::Instant};

// Atomic 原子性,避免数据竞争
static COUNTER: AtomicUsize = AtomicUsize::new(0); // Atomic 内部实现了可变性



fn main() {
    let start = Instant::now();

    let mut handles = Vec::new();
    for _ in 0..1000 {
        let h = thread::spawn(||{
            for _ in 0..1000 {
                COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
        });
        handles.push(h);
    }

    handles.into_iter().for_each(|h|h.join().unwrap());
    println!("Total: {}",COUNTER.load(std::sync::atomic::Ordering::Relaxed)); // 1000 000

    let elapsed = start.elapsed();
    println!("Elapsed time: {}", elapsed.as_micros());
}