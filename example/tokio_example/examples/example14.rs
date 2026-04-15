// cargo add rayon
// cargo add tokio -F full

// cargo run --example example14
use std::time::Duration;
use tokio::sync::mpsc; // 异步多生产者单消费者通道
use rayon::ThreadPoolBuilder; 

#[tokio::main]
async fn main() {
    // 通道初始化
    /*
        tx 用于发送待处理的任务（Task 枚举）
        tx_reply 用于回传计算结果（i32 类型）
        有界通道提供背压：当缓冲区满时，send 会异步等待直到有空位，防止生产者过载
    */
    let (tx, mut rx) = mpsc::channel::<Task>(20);
    let (tx_reply, mut rx_reply) = mpsc::channel::<i32>(20);

    // 线程池初始化
    let pool = ThreadPoolBuilder::new()
        .num_threads(4) // 最大并行任务数
        .build()
        .unwrap();

    // 当前 Tokio 运行时句柄
    // 
    let handle = tokio::runtime::Handle::current();
    // Handle 是对 Tokio 运行时的引用，可以从任意线程向该运行时提交异步任务（spawn）
    // 这里将其 clone 后移入 Rayon 线程的闭包中，使得在 Rayon 线程内能够安全地将结果发送回 Tokio 的异步世界
    tokio::spawn(async move { // tokio
        while let Some(task) = rx.recv().await {
            let tx_reply = tx_reply.clone();
            let handle = handle.clone();
            pool.spawn(move || { // rayon
                // 直接解构，消除 irrefutable_let_patterns 警告
                let Task::Calculate(n) = task;
                let result = n * n;
                handle.spawn(async move { // tokio
                    tx_reply.send(result).await.unwrap(); // 发送结果
                });
            });
        }
    });

    tokio::spawn(async move {
        while let Some(result) = rx_reply.recv().await { // 接收结果
            println!("Received result: {}", result);
        }
    });

    // 发送任务
    let mut num = 1;
    loop {
        tokio::time::sleep(Duration::from_millis(500)).await;
        tx.send(Task::Calculate(num)).await.unwrap(); // 发送任务
        num += 1;
    }
}

enum Task {
    Calculate(i32),
}