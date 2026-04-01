/*
    Tokio
    一个用于执行异步代码的多线程运行时
    标准库的异步版本
    一个庞大的库生态系统
*/


// 单线程模式
// cargo add tokio -F full
// cargo install cargo-expand

// cargo run --example example1
// use tokio::runtime;

async fn hi() {
    println!("Hello tokio!");
}
// 利用宏简化写法
// #[tokio::main] // 多线程写法
#[tokio::main(flavor = "current_thread")] // 单线程写法
async fn main() {
    hi().await;
    // 原始写法
    // let rt = runtime::Builder::new_current_thread()
    // .enable_all()
    // .build()
    // .unwrap();
    
    // rt.block_on(hi());

}
