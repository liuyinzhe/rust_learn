
/*
    在执行计算时会阻塞线程池, 
    只有当让出控制权时(yield,await或者等待一个I/O任务结束时)才能运行
*/


use std::{thread, time::Duration};

// cargo run --example example4


async fn hello(task: u64, time: u64) {

    println!("Task {task} started on {:?}", std::thread::current().id());
    // std 的sleep函数会阻塞线程池, 导致其他任务无法执行
    // std::thread::sleep(Duration::from_millis(time));

    // tokio sleep await 异步等待,await 让出当前任务的控制权, 并切换到其他任务
    tokio::time::sleep(Duration::from_millis(time)).await;
    println!("Task {task} finished......");
}


#[tokio::main] // 多线程写法
async fn main() {
    let _ = tokio::join!(
        // 串行执行
        hello(1, 100),
        hello(2, 200),
        hello(3, 300),
        hello(4, 400),
        hello(5, 500),

        // // 并行执行
        // tokio::spawn(hello(1, 100)),
        // tokio::spawn(hello(2, 100)),
        // tokio::spawn(hello(3, 100))
    );
    println!("Finished");


   }
