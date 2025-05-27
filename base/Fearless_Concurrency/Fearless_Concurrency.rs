
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

    线程actors 通过发送包含数据的消息来互相通信
    
    Rust 的标准库提供了通道(channel)的实现

    通道
    channel
    通道是一种程序设计概念，用于在不同线程之间发送数据
    发送端(transmitter) 接收端(receiver)
    
    当通道的任意一端被丢弃时，通道被关闭

*/

use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::Duration;
fn main(){
    let (tx,rx) = mpsc::channel();
    thread::spawn(move || { // move 转移所有权
        // let val = String::from("hi");
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),  
        ];
        for val in vals{
            tx.send(val).unwrap(); // 发送
            thread::sleep(Duration::from_secs(1));
        }

        /*
        send 接受一个要发送的值参数,丢弃时返回错误
        返回一个Result<T,E>
         */
    });

    
    // let received = rx.recv().unwrap(); // recv 阻塞当前线程，直到接收到一个值
    for received in rx {
        // try_recv 方法 不会阻塞，二十立即返回一个Result
        // 如果当前有消息可用，则返回Ok包含该内消息
        // 如果当前没有信息，则返回Err
        println!("Got: {received}");
    }

}

/*
    共享数据
    让多个线程访问相同的共享数据

    在某种程度上
        任何编程语言中的通道都类似于单一所有权
        共享内存并发就像多重所有权:多个线程可以同时访问相同的内存位置

    Mutex 互斥锁

    互斥锁在任何给定时间只允许一个线程访问某些数据
    要访问互斥锁中的数据,线程必须请求获取互斥锁的锁(所有权)
    锁时互斥锁的一种数据结构,用于跟踪谁当前拥有对数据的独占访问权
    互斥锁被描述为通过锁定系统来保护它所持有的数据

    Mutex 两条规则
    使用数据之前，必须尝试获取锁
    当使用完互斥锁保护的数据后，必须解锁数据,以便其他线程可以获取锁

*/
use std::sync::Mutex;
// 单线程
fn main(){
    let m = Mutex::new(5);
    // 作用域
    {
        let mut num = m.lock().unwrap(); // 返回LockResult<MutexGuard<T>> 
        *num = 6; // 通过解引用修改被保护的值，解引用智能指针
    }// MutexGuard实现了Drop trait，离开作用域自动释放锁

    println!("m = {m:?}"); // m = Mutex { data:6, poisoned: false, ..}

}
/*
使用Arc<T>进行原子引用计数

Arc<T> 是一种类似Rc<T>的类型,可以安全地在并发环境中使用

A 代表 atomic(原子性), 意味着它是一种原子引用计数类型
参考标准库文档中的 std::sync::atomic

缺点:线程安全会带来性能损失
*/

// 10个线程
use std::sync::Mutex;
use std::thread;
use std::sync::Arc;

fn main() {
    let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter) // 克隆counter
        let handle = thread:spawn(move || { // move 会移动counter 所有权，导致第二个循环无法使用counter // 基于Arc 智能指针和克隆解决
            let mut num = counter.lock().unwrap(); // 获得锁定的数据智能指针

            *num + = 1;// 解引用 修改数值
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();// thread join 等待多线程执行完毕
    }
    println!("Result: {}",*counter.lock().unwrap());
}

/*
    Sync & Send traits

    Rust 大多数并发功能都是标准库的一部分，而不是语言本身
    可以编写自己的并发功能或使用第三方库
    个并发概念: std::marker traits  ---- Sync 和Send # 标记

    Send trait

    Send(marker trait) 所有权可以在线程之间转移
    几乎所有Rust类型都是Send
    但有一些例外,例如Rc<T> 不是Send
        Rc<T>仅用于单线程情况
    
    Rust 的类型系统和trait约束确保不会意外地将非Send类型跨线程发送
    完全有Send 类型组成的任何类型也自动标记为Send
    几乎所有原始类型都是Send,原始指针除外


    Sync(marker trait): 可以安全地从多个线程引用实现该trait的类型

    如果&T 时Send,则类型T是Sync
       即该引用可以安全的发送到另外一个线程

    原始类型是Sync

    完全由Sync类型做成的类型也是Sync
*/

use std::thread;

const X: i32 = 42;

fn main() {
    let x_ref = &X; // 由于是i32 数字,赋值时copy
    let ref_x_thread = x_ref;
    let ref_x_main = x_ref;

    println!("X ref: {x_ref}");
    println!("ref_x_thread: {ref_x_thread}");
    println!("ref_x_main: {x_rref_x_mainef}");

    let t1 = thread::spawn(move || {
        println!("In thread: {}",ref_x_thread); // &i32 --> Send 对i32的引用 是 Send
    });
    println!("Main thread: {}", ref_x_main);
    
    t1.join().unwrap();
}

/*
    线程安全性与Sync
    Sync 是Rust中最接近 "线程安全"的概念
        "线程安全"指特定数据可以被多个并发线程安全使用

    分开Send和Sync 特性(trait)的原因:一个类型可能是其中之一,两者都是，或者两者都不是
        Rc<T>: 既不是Send 也不是Sync
        RefCell<T>: 是Send(如果T是Send),但不是Sync // 代表数据的唯一所有权，但通过内部可变性模式允许在不可变引用下修改数据,支持运行时检查的可变借用(通过borrow_mut)，即使外部表现为不可变
        Mutex<T>: 是Send 也是Sync,可用于多线程共享访问 // 互斥锁
        MutexGuard<'a,T>: 是Sync(如果T是Sync)但不是Send //互斥锁.lock() 返回值(智能指针)LockResult<MutexGuard<T>> 

    手动实现Send和Sync
    手动实现涉及到实现unsafe Rust 代码
    
*/
