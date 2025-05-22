
/*
    // 并发
    Concurrent Programming  程序的不同部分可独立的执行
    // 并行
    Parallel Programming    程序的不同部分可同时执行

*/

/*
    使用多线程公式运行代码

    代码在一个进程(Process)中运行,在程序内部，独立部分的功能独立同时运行，称为线程

    多线程可导致的问题

    竞态条件(race condition)
        线程以不一致的顺序访问数据或资源
    死锁(Deadlocks)
        两个线程互相等待，导致两个线程都无发继续(资源锁?)
    
*/

/*
    thread::spawn 创建新线程
    thread::scope
*/
use std::thread;
use std::time::Duration;
fn main(){
    let handle = thread::spawn(||{ // 参数为闭包
        for i in 1..<10{
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();// 阻塞主线程，等待都运行完成
}



use std::thread;
fn main(){

    let v = vec![1,2,3];
    let handle = thread::spawn( move|| { // move 将v 所有权移动到闭包中
        println!("Here's a vecter:{v:?}");
    });

    handle.join().unwrap();// 阻塞主线程，等待都运行完成
}

/*
    使用信息传递在线程之间传输数据

    消息传递
    message passing

    线程actors  通过发送包含数据的消息来互相通信
    
    Rust 的标准库提供了通道(channel)的实现
*/
