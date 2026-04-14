// cargo add rayon
// cargo add tokio -F full

// cargo run --example example14
use std::time::Duration;
use tokio::sync::mpsc;
use rayon::ThreadPoolBuilder;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<Task>(20);
    let (tx_reply, mut rx_reply) = mpsc::channel::<i32>(20);

    let pool = ThreadPoolBuilder::new()
        .num_threads(4) // 最大并行任务数
        .build()
        .unwrap();

    let handle = tokio::runtime::Handle::current();

    tokio::spawn(async move {
        while let Some(task) = rx.recv().await {
            let tx_reply = tx_reply.clone();
            let handle = handle.clone();
            pool.spawn(move || {
                // 直接解构，消除 irrefutable_let_patterns 警告
                let Task::Calculate(n) = task;
                let result = n * n;
                handle.spawn(async move {
                    tx_reply.send(result).await.unwrap();
                });
            });
        }
    });

    tokio::spawn(async move {
        while let Some(result) = rx_reply.recv().await {
            println!("Received result: {}", result);
        }
    });

    let mut num = 1;
    loop {
        tokio::time::sleep(Duration::from_millis(500)).await;
        tx.send(Task::Calculate(num)).await.unwrap();
        num += 1;
    }
}

enum Task {
    Calculate(i32),
}