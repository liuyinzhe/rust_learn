
// 多线程模式
// 默认, Tokio 会为每个cpu核心启动一个线程(可自行控制)
// 每个线程都有自己独立的任务队列(task list)
// 每个线程都有自己的反应器(reactor),即事件循环
// 每个线程都支持工作窃取(work stealing)

// cargo run --example example2
use tokio::runtime;
async fn hi() {
    println!("Hello tokio!");
}
// 利用宏简化写法
// #[tokio::main] // 多线程写法
// async fn main() {
fn main() {

    let rt = runtime::Builder::new_multi_thread()
    .worker_threads(10) // 启动 10 个线程
    .thread_stack_size(5*1024*1024) // 每个线程的栈大小 5MB
    .event_interval(20) // 事件循环的事件间隔 20ms
    .max_blocking_threads(256) // 最大阻塞线程数 256
    .enable_all() // 启用所有功能
    .build()
    .unwrap();
    
    rt.block_on(hi());

}
