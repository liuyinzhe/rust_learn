
/*
    spawn_blocking
    如果向Tokio 里执行阻塞操作
      到没有async 接口设备的I/O操作
      CPU 密集型任务
      ...其它无法async的任务
    Tokio提供的spawn_blocking 函数

*/


use std::time::Duration;
use tokio::task::spawn_blocking;
use tokio::sync::Semaphore;
use std::sync::Arc;
// 信号量(Semaphore)来限制同时执行的spawn_blocking任务的数量。

// cargo run --example example5


async fn delay(task: u64, time: u64,semaphore: Arc<Semaphore>) {
    println!("Task {task} starts.");
    // 获取信号量许可，若已满则等待
    let _permit = semaphore.acquire().await.unwrap();
    // 阻塞任务执行期间，许可一直被持有

    // spawn_blocking 执行阻塞操作
    let result = spawn_blocking(move || {
        std::thread::sleep(Duration::from_secs(time));
        println!("blocking...");
    }).await; // 去掉await 后,后台运行,不阻塞主任务
    println!("{:?}", result);
    println!("Task {task} ends.");
    // _permit 在此处离开作用域，自动释放许可
}


#[tokio::main] // 多线程写法
async fn main() {
    let max_concurrent = 2; // 限制同时运行的阻塞任务数量为 2
    let semaphore = Arc::new(Semaphore::new(max_concurrent));
    tokio::join!(
        delay(1, 2, semaphore.clone()),
        delay(2, 2, semaphore.clone()),
        delay(3, 2, semaphore.clone()),
        delay(4, 2, semaphore.clone()),
    );
    println!("Main Finished");


   }
