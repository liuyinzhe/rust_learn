// cargo add rayon
// cargo add tokio -F full

/*
    Mutex
    Async Rust

    Atomic 变量
    Atomic 变量不受Async 影响

    Mutex
    async 里可以使用 标准库的 Mutex

    但是 Mutex 在 async 环境中有两个问题:
    如果Mutex 被争抢，会阻塞整个线程
    不能把标准库的 Mutex 安全地跨await 使用

    Tokio 的 async Mutex
    与标准库使用方式几乎一样
    枷锁需要 await

    RwLock
    用法与标注库的一致
    read(),write()需要await
*/

// cargo run --example example16

// use std::sync::Mutex;

// static COUNTER: Mutex<u32> = Mutex::new(0);

// async fn add(n: u32) -> u32 {
//     n + 1
// }

// async fn incr() {
//     let mut counter = COUNTER.lock().unwrap();
//     // *counter += 1;
//     *counter = add(*counter).await;
// }

// #[tokio::main]
// async fn main() {
//     tokio::join!(incr(), incr(),incr());
//     println!("Counter = {}", *COUNTER.lock().unwrap());
// }

use std::{time::Duration, sync::LazyLock};

use tokio::{sync::Mutex,time::sleep};

static DATA: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));

async fn task1() {
    println!("Task1 try lock");
    let _guard = DATA.lock().await;
    println!("Task1 locked, sleep 5s");
    sleep(Duration::from_secs(5)).await;
    println!("Task1 done");
}

async fn task2() {
    sleep(Duration::from_millis(100)).await;
    println!("Task2 try lock");
    let _guard = DATA.lock().await;
    println!("Task2 locked");
    println!("Task2 done");
}

#[tokio::main]
async fn main() {
    tokio::join!(task1(), task2());
}