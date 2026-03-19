// examples/example4.rs
// cargo run --example example4
fn main() {
    // 线程池join
    let pool = rayon::ThreadPoolBuilder::new()
    .num_threads(4)
    .build()
    .unwrap();

    let func = || println!("Hello!");
    // 只支持两个任务
    pool.join(func, func);
}