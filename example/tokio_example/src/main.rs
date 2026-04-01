// cargo add futures
// cargo add tokio -F full
// cargo run --bin tokio_example

use futures::{executor::block_on, join};


async fn hi() {
    println!("Hello!");
    // hello().await; // async fn 中调用 async fn 只需 添加.await 即可

}

// 1. async fn 可以执行 non-async fn
// 2. non-async fn 不可以执行 async fn,除非有 executor

async fn hello() {
    println!("Hello, world!");
    hello_sync(); // 可以执行非async fn

}

fn hello_sync() {
    println!("Hello sync!");
}

async fn do_mul() {
    join!(hi(),hello()); // 可以并行执行2个 async fn
    let sum = add(1,2).await; // async 上下文中，调用asynchronous function只需要添加 .await 即可
    println!("Sum: {}", sum);
    let (a,b) = join!(add(1,2),add(3,4));
    println!("Sums: {a}, {b}");
}

async fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let func = hi(); // executor
    // Run a future to completion on the current thread.
    // cooperative multitasking
    block_on(func); // 单线程执行 async fn , 直到完成 
    block_on(do_mul());

}
