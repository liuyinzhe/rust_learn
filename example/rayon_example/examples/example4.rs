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
    // join 是 Rayon 提供的一个用于并行执行两个任务并等待两者完成的方法
    pool.join(func, func);
}