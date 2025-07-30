use std::{thread, time::Instant};

static mut COUNTER: usize = 0;



fn main() {
    let start = Instant::now();

    let mut handles = Vec::new();
    for _ in 0..1000 {
        let h = thread::spawn(||{
            for _ in 0..1000 {
                unsafe { // 数据竞争 
                    COUNTER += 1;
                }
            }
        });
        handles.push(h);
    }

    handles.into_iter().for_each(|h|h.join().unwrap());
    println!("Total: {}",unsafe {COUNTER}); // 数据竞争 导致结果不为 1000 000

    let elapsed = start.elapsed();
    println!("Elapsed time: {}", elapsed.as_micros());
}