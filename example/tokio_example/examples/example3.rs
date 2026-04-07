
// Executor 执行器
// Reactor  反应器 暂停任务存放和恢复
// 并发 vs 并行

// Concurrency vs Parallelism
// 时间分片 Time Slicing
// 异步挂起/恢复
// Async
// Suspend/Resume

// 系统线程 vs 绿色线程
// OS Thread vs Green Thread


/*
    Runtime
    I/O
    File System
    Network
    Time
    Process
*/

// 常用函数
// spawn  // 生成一个新的异步任务, spawn 添加异步任务到任务池中
// join!  // 等待多个任务完成, 并返回结果
// yield_now // 暂停当前任务, 并切换到其他任务

// use tokio::task::JoinSet;

// cargo run --example example3
// use tokio::time::Duration;

async fn _hello() {
    // tokio::time::sleep(Duration::from_secs(1)).await;
    // for i in 0..2500 {
    //     let _ = i *25;
    // }
    println!("Hello tokio!");
}

async fn _run(){
    for i in 0..10{
        println!("{i}"); 
        tokio::task::yield_now().await; // 遍历过程中让出控制权,暂停当前任务, 并切换到其他任务
    }
}

async fn _add(a: i32,b: i32) -> i32 {
    println!("{}", a + b);
    a + b
}

// 利用宏简化写法
#[tokio::main] // 多线程写法
async fn main() {
    // tokio::spawn(run());// 生成一个新的异步任务, spawn 添加异步任务到任务池中
    // hello().await; // 系统线程

    // join!等待多个任务完成, 并返回结果
    // let result = tokio::join!(add(1,2), add(3,4));
    // println!("{:?}", result);

    // JoinSet
    // let mut set = JoinSet::new(); // 运行时任务集合
    // for i in 0..10 {
    //     set.spawn(add(i,2));
    // }

    // // join_next 等待任意任务完成, 并返回结果
    // while let Some(result) = set.join_next().await {
    //     println!("{:?}", result);
    // }

    // 
    let _ = tokio::join!(
        // 如果没有.await ,任务不让出控制权，不给暂停
        tokio::spawn(_hello()),
        tokio::spawn(_run()),
        tokio::spawn(_run())

    );
    println!("Finished");


   }
