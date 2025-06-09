
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
use std::thread::{self, sleep};
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

    
    // let received = rx.recv().unwrap(); // recv 阻塞当前线程，直到接收到一个值 // recv 类似迭代器
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

/*
    异步编程
    Asynchronous Programming

    并行性 Parallelism:同时执行多个操作
    并发性 Concurrency: 在操作间进行切换
    阻塞操作 Blocking Operations:阻止程序继续执行直到操作完成
    非阻塞操作 Non-blocking Operations:允许程序在等待时执行其他任务

    操作类型
    Type of Operations
    CUP 密集型(CPU-boun):受处理器能力限制(如视频导出)
    IO 密集型(IO-bound):受输入/输出速度限制(如文件下载)

    并行与并发
    Parallelism and Concurrency

    并发性(Concurrency): 一个执行单元处理多个任务,通过任务切换实现
    并行性(Parallelism): 多个执行单元同时处理不同任务
    串行性(Serial Work): 任务按特定顺序一个接一个执行完成
*/

/*
    Futures 和异步语法
    Future and the Async Syntax

    核心元素
    Core Elements

    Future(未来量): 一个可能现在还未准备好(就绪),但将来会准备好(就绪)的值
        在其它语言中也称为 task或者 promise
        在Rust中,Future 是实现了Future trait 的类型
    
    async 关键字:用于代码块或者函数,来表示可被中断和恢复
        将函数或代码块转换为返回Future 的类型
    await 关键字: 用于等待Future准备好(就绪)
        提供暂停和恢复的点
        轮询(polling)是检查Future值是否可用的过程

    
    Future 的特点
    Future Characteristics

    Rust 编译器将 async/await 代码转为使用 Fuure trait的等效代码
        类似于for 循环被转换为使用Iterator trait
    开发者可以自定义数据类型实现Future trait
        提供统一接口但允许不同的异步操作实现
    

    trpl 整合了我们需要的类型、trait和函数，主要来自futures 和 tokio两个核心异步库

    目标:专注于异步编程学习，避免生态系统干扰

    工具: 使用trpl 库(The Rust Programming Language)
        整合 futures 和tokio的核心功能
        futures: 异步实验的官方家园，定义了Future特性
        tokio: 最流行的异步运行式，广泛用于Web开发
    设计:
        trpl 重导出类型、函数和trait,简化学习
        隐藏复杂细节，专注于异步核心
*/

use trpl::{Html,Either};

fn main(){
    let args: Vec<String> = std::env::args().collect();

    // trpl::run( // main 中 启用async
    //     async {
    //         let url = &args[1];
    //         match page_title(url).await {
    //             Some(title) => println!("The title for {url} was {title}"),
    //             None => println!("{url} had no title"),
    //         }
    //     } // async块 作用域
    // ) // run 
    trpl::run( async {
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);

        let (url,maybe_title) = 
            match trpl::race(title_fut_1,title_fut_2).await { //trpl::race 竞争,获取最先完成得的
                Either::Left(left) => left,// 看左右 Future 那个先完成
                Either::Right(right) => right,
            };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its pag title is '{title}'"),
            None => println!("Its title could not be parsed."),
        }
    })

}

async fn page_title(url:&str) -> (&str,Option<String>) { //async 来表示可被中断和恢复 // impl Future 实现了Future trait
    let response = trpl::get(url).await.text().await; // Future 等待,await执行// 获取返回文本内容 Future 等待,await执行
    let title = Html::parse(&response) // 函数所有权转移,使用引用&
        .select_first("title") // 选择 第一个title元素
        .map(|title_element| title_element.inner_html()); // map 获取 title 元素内容inner_html
    (url,title)
}

/*
    使用 Async 实现并发
    Applying Concurrency with Async


*/

use trpl
// fn main() {
//     trpl::run(
//         async {
//             let handle = trpl::spawn_task(async { // spawn_task 创建一个任务
//                 for i in 1..10 {
//                     println!("hi number {i} from the first task");
//                     trpl::sleep(Duration::from_millis(500)).await; // 等待500毫秒-半秒
//                 }
                
//             });
            
//             for i in 1..5 {
//                     println!("hi number {i} from the second task");
//                     trpl::sleep(Duration::from_millis(500)).await; // 等待500毫秒-半秒
//             }
//             handle.await.unwrap();
//         }
//     );
// }

// 同上
fn main() {
    trpl::run(
        async { // async 块内 两个任务实现并行
            let fut1 = async { // spawn_task 创建一个任务
                for i in 1..10 {
                    println!("hi number {i} from the first task");
                    trpl::sleep(Duration::from_millis(500)).await; // 等待500毫秒-半秒
                }
                
            };
            
            let fut2 = async {
                for i in 1..5 {
                        println!("hi number {i} from the second task");
                        trpl::sleep(Duration::from_millis(500)).await; // 等待500毫秒-半秒
                }
            };
            trpl::join(fut1,fut2).await;
        }
    );
}
use std::time::Duration;
fn main() {
    trpl::run( // 整体阻塞 block_on 
        async { // 不会阻塞
            let (tx,mut rx) // tx发送端, rx接收端
            = trpl::channel(); // 建立通道
            // tx.clone(); 发送端可以克隆

            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),

            ];
            // let val = String::from("hi");
            // tx.send(val).unwrap(); // unbound 
            let tx_fut = async move { //使用move 转移到async作用域内,出了作用域，自动销毁，rx.recv()才能中断监听
                for val in vals{
                    tx.send(val).unwrap();
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };

            // let received = rx.recv().await.unwrap();
            // println!("收到: {received}");

            let rx_fut = async {
                while let Some(value) = rx.recv().await { // rx.recv()一直监听,tx_fut 在作用域内销毁后，才会关闭channel;使用move 转移到async作用域内
                    println!("received '{value}' ");
                }
            };
            // 两个一起执行；输出只在rx_fut 中执行
            trpl::join(tx_fut,rx_fut).await;

        }
    );

}

/*
    处理任意数量的Future
    Workingwith Any Number of Futures

*/
use std::pin::{pin,Pin};
use std::time::Duration;
fn main() {
    trpl::run( // 整体阻塞 block_on 
        async { // 不会阻塞
            let (tx,mut rx) // tx发送端, rx接收端
            = trpl::channel(); // 建立通道
            let tx1 = tx.clone(); //发送端可以克隆
            // pin!宏 包装async 块
            let tx1_fut = pin!(async move {
                let vals = vec![
                    String::from("hi"),
                    String::from("from"),
                    String::from("the"),
                    String::from("future"),
                ];
                for val in vals{
                    tx1.send(val).unwrap();
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            });


            // let val = String::from("hi");
            // tx.send(val).unwrap(); // unbound 
            // let tx_fut = async move { //使用move 转移到async作用域内,出了作用域，自动销毁，rx.recv()才能中断监听
            //     for val in vals{
            //         tx.send(val).unwrap();
            //         trpl::sleep(Duration::from_millis(500)).await;
            //     }
            // };

            // let received = rx.recv().await.unwrap();
            // println!("收到: {received}");

            

            // 第二个发送端 // pin!宏 包装async 块
            let tx_fut = pin!(async move {
                let vals = vec![
                    String::from("hi"),
                    String::from("from"),
                    String::from("the"),
                    String::from("future"),
                ];
                for val in vals{
                    tx.send(val).unwrap();
                    trpl::sleep(Duration::from_millis(1500)).await;
                }
            });

                        // 接收端 // pin!宏 包装async 块
            let rx_fut = pin!(async move { // Future<Output = ()> async 块返回的类型
                while let Some(value) = rx.recv().await { // rx.recv()一直监听,tx_fut 在作用域内销毁后，才会关闭channel;使用move 转移到async作用域内
                    println!("received '{value}' ");
                }
            });

            // Pin 包装类型 ,统一类型// dyn 动态
            let futures: Vec<Pin<&mut dyn Future<Output=()>>>  = 
                vec![tx1_fut,rx_fut,tx_fut];

            // 两个一起执行；输出只在rx_fut 中执行
            // trpl::join3(tx1_fut,tx_fut,rx_fut).await;
            // 要求传入内容一致，用于动态数量的Future
            trpl::join_all(futures).await;
        }
    );

}



// 不同返回值的
use std::pin::{pin,Pin};
use std::time::Duration;

fn main() {
    trpl::run( // block_on
        async {
            let a = async{1u32};
            let b = async{ "Hello!" };
            let c = async{ true };
            
            // join!() 允许内容类型不一致,用于处理固定数量的Future
            let (a_result,b_result,c_rsult) = trpl::join!(a,b,c);
            println!("{a_result},{b_result},{c_rsult}");

        }
    );
}

// 竞争的Futures
// Racing Futures

use std::thread;
use std::time::Duration;


fn main() {
    trpl::run(
        async {
          let  a = async {
            println!("'a' started");
            // 阻塞步骤
            slow("a", 130);
            trpl::yield_now().await; // 异步控制权交给 异步运行时 # 线程切换位置
            slow("a", 10);
            trpl::yield_now().await; // 异步控制权交给 异步运行时 # 线程切换位置
            slow("a", 120);
            trpl::yield_now().await; // 异步控制权交给 异步运行时 # 线程切换位置
            //trpl::sleep(Duration::from_millis(100)).await; // 异步 sleep, await 线程切换位置, 异步控制权交给 异步运行时
            println!("'a' finished");
          };

          let  b = async {
            println!("'b' started");
            
            // 阻塞步骤
            slow("b", 30);
            trpl::yield_now().await; // 异步控制权交给 异步运行时 # 线程切换位置
            slow("b", 10);
            trpl::yield_now().await; // 异步控制权交给 异步运行时 # 线程切换位置
            slow("b", 20);
            trpl::yield_now().await; // 异步控制权交给 异步运行时 # 线程切换位置
            //trpl::sleep(Duration::from_millis(50)).await; // 异步 sleep,  await 线程切换位置, 异步控制权交给 异步运行时
            println!("'b' finished");
          };

          trpl::race(a,b).await; // trpl::race() 竞争运行 Racing Future // 任意执行完毕,程序直接停止
          // 第一个参数式第一个执行

        }
    );
}

fn slow(name: &str,ms: u64) {
    thread::sleep(Duration::from_millis(ms)); // 顺序执行阻塞 sleep
    println!("'{name}' 运行了 {ms}ms");
}

// trpl::yield_now().await; // 最高效
// 异步控制权交给 异步运行时 # 线程切换位置

use std::thread;
use std::time::{Duration,Instant};

fn main() {
    trpl::run(
        async {
            let one_ns = Duration::from_nanos(1);

            let start = Instant::now(); // 当前时间
            async {
                for _ in 1..1000 {
                    trpl::sleep(one_ns).await;
                }
            }.await;
            // 输出用时
            let time = Instant::now() - start;
            println!(
                "'sleep'版本在{}秒后完成。",
                time.as_secs_f32()
            );


            let start = Instant::now(); // 当前时间

            async {
                for _ in 1..1000 {
                    trpl::yield_now().await;
                }

            }.await;
            // 输出用时
            let time = Instant::now() - start;
            println!(
                "'yield'版本在{}秒后完成。",
                time.as_secs_f32()
            );

        }
    )
}
/*
'sleep'版本在15.23684秒后完成。
'yield'版本在0.0001741秒后完成。

异步控制权交给 异步运行时 # 线程切换位置
最好使用 trpl::yield_now().await; 速度快
*/



// 异步 函数
use std::time::Duration;
//use std::thread;
use trpl::Either;
fn main() {
    trpl::run(
        async {
            let slow = async {
                //thread::sleep(Duration::from_secs(5)); // 因为没有 await 切换, 没有进行超时比较，直接返回"I finished!"成功
                trpl::sleep(Duration::from_secs(5)).await;
                "I finished!"
            };

            match timeout(slow,Duration::from_secs(2)).await {
                Ok(message) => println!("Succeeded with '{message}'"),
                Err(duration) => {
                    println!("Failed after {} seconds", duration.as_secs())
                }
            }
        }
    );
}
// 异步函数
async fn timeout<F: Future>(
    future_to_try:F,
    max_time: Duration, // 形参 max_time 从未使用
) -> Result<F::Output,Duration> { // Future::Output

    // 竞争运行
    match trpl::race(future_to_try,trpl::sleep(max_time)).await { // use trpl::{Html,Either};形参,未使用
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),

    }
}
// Result:
// Failed after 2 seconds

/*
    Streams
    Futures in Sequence
    
    Stream 就像异步版本的 iterator  recv() trpl::Receiver
    可以从任何 iterator 来创建 Stream

    StreamExt
    Ext 是Rust社区中使用另外一个trait扩展某个trait的常见模式

    Stream trait 定义了一个低级接口，有效地结合了Iterator 和Future traits
    StreamExt 在Stream之上提供了一组更高级的API,包括next方法以及类似于Iterator trait 提供的其他使用方法

    Stream 和 StreamExt 尚未称为Rust 标准库的一部分，但大多数生态系统中crate 使用相同的定义
*/

// StreamExt examle 1
use trpl::StreamExt;
fn main() {
    trpl::run(
        async {
            let values:[i32;10] = [1,2,3,4,5,6,7,8,9,10];
            let iter = values.iter().map(| n:&i32 | n*2 ); // iter() 列表转迭代器,使用map遍历转换数值内容
            let mut stream = trpl::stream_from_iter(iter);

            while let Some(value) = stream.next().await {
                println!("The value was: {value}");
            }
        }
    ); //初始化异步运行时

}


// StreamExt examle 2
use trpl::StreamExt;
fn main() {
    trpl::run(
        async {
            let values = 1..101;
            let iter = values.map(| n | n*2 ); 
            let stream = trpl::stream_from_iter(iter);

            let mut filtered = 
                stream.filter(|value |value % 3 ==0|| value % 5 ==0); //3 或者 5 的倍数

            while let Some(value) = filtered.next().await {
                println!("The value was: {value}");
            }
        }
    ); //初始化异步运行时

}
//

/*
    Composing Streams
    组合流

*/
use std::{pin::pin,time::Duration};
use trpl::{ReceiverStream,Stream,StreamExt};

fn main() {
    trpl::run( async {
        let mut messages = 
            pin!(get_messages().timeout(Duration::from_millis(200))); // 流添加超时，来自StreamExt
            // pin! 固定后 Stream才能轮值(await)
        while let Some(result) = messages.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem {reason:?}"),
            }
            

        }
    });
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx,rx) = trpl::channel(); // 异步通信通道

    trpl::spawn_task( async move {
        let messages = ["a","b","c","d","e","f","g","h","i","j"];
        for (index,message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index %2 == 0 { 100 }else{300};
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            tx.send(format!("message: '{message}'")).unwrap();

        }
    });

    ReceiverStream::new(rx) // 将异步通信接收器rx 转为具有next() 方法的流(Stream)
}


// 合并流

use std::{pin::pin,time::Duration};
use trpl::{ReceiverStream,Stream,StreamExt};

fn main() {
    trpl::run( async { // 启动异步运行时
        // 使用pin固定流在内存中的位置
        let mut messages = pin!(get_messages().timeout(Duration::from_millis(200))); 
        let intervals = get_intervals()
            .map(|count| format!("Interval: {count}")) // 操作内容格式化字符串
            .throttle(Duration::from_millis(100)) // 截留,限制调用的频率
            .timeout(Duration::from_millis(10)); // 设置任务超时时间,全部是相同的timeout 类型
        // 合并两个流任务，一起线程切换跑
        let merged = messages.merge(intervals).take(20); // 合并相同的 timeout 类型 的流 // take(20) 限制从流中接收数据的数量，取20个停止 类似head 意思
        let mut stream = pin!(merged); // ReceiverStream 流,允许通过 next().await 消费数据

        while let Some(result) = stream.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem {reason:?}"),
            }
        }
    })
}

fn get_intervals() -> impl Stream<Item = u32> { // impl 实现了Stream 泛型返回值(返回泛型内容为u32 整数)
    let (tx,rx) = trpl::channel(); // 异步通信通道

    trpl::spawn_task( async move { //启动新的异步任务 // async move 块获取环境变量的所有权
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;
            if let Err(send_error) = tx.send(count){
                eprintln!("Could not send Interval '{count}':{send_error}");
                break;    
            }
        }
    });

    ReceiverStream::new(rx) // 将异步通信接收器rx 转为具有next() 方法的流(Stream)
}

fn get_messages() -> impl Stream<Item = String> { // impl 实现了Stream trait 泛型返回值(返回泛型内容为String)
    let (tx,rx) = trpl::channel(); // 异步通信通道

    trpl::spawn_task( async move {
        let messages = ["a","b","c","d","e","f","g","h","i","j"];
        for (index,message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index %2 == 0 { 100 }else{300};
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            if let Err(send_error) = tx.send(format!("message: '{message}'")){
                eprintln!("Cannot send message '{message}':{send_error}");
                break;    
            }
        }
    });

    ReceiverStream::new(rx) // 将异步通信接收器rx 转为具有next() 方法的流(Stream)
    /*
        将通道接收端 rx 包装成 ReceiverStream
        ReceiverStream 是实现了 Stream trait 的适配器
        允许通过 next().await 消费数据
     */
}

/*
特性	trpl::spawn_task	trpl::run
执行方式	非阻塞	阻塞
调用位置	可在异步代码内部调用	通常在 main 函数调用
作用	创建新任务	启动整个运行时
返回值	JoinHandle	程序退出码
任务关系	子任务	根任务
使用频率	可多次调用	每个程序调用一次
线程行为	立即返回	阻塞当前线程
*/



/*
    Traits for Async
    异步主要的Traits

*/

// Future trait
use std::pin::Pin;
use std::task::{Context,Poll};

// Future 轮询 Poll 状态
pub trait Future {
    type Output;
    // Pin 内存中固定，因为有内部的自身引用，保证Future 可以放心引用
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T), // 准备就绪
    Pending,  // 未就绪
}

// poll 使用 // Future 轮询 Poll 状态

async fn page_title(url:&str) -> (&str,Option<String>) { //async 来表示可被中断和恢复 // impl Future 实现了Future trait
    let response = trpl::get(url).await.text().await; // Future 等待,await执行// 获取返回文本内容 Future 等待,await执行 // await 后面调用poll 轮询
    let title = Html::parse(&response) // 函数所有权转移,使用引用&
        .select_first("title") // 选择 第一个title元素
        .map(|title_element| title_element.inner_html()); // map 获取 title 元素内容inner_html
    (url,title)
}


// Pin & Unpin
// Pin 内存中固定，因为有内部的自身引用，保证Future 可以放心引用
// 实现了 Unpin trait 的 future 时，也就实现了 Future
/*
    Pin 
    Pin是 针对(类) 指针类型(如 & & mut Box和 Rc) 的包装器
    它纯粹是编译器可以用来强制约束指针使用的工具 // 用于引用指针类型

    与Rust中 大多数其他类型不同，Rust为异步块创建Future
    可能最终在任何给定的变体的字段中 包含对自身的引用

    Unpin & !Unpin
    Unpin 是一个标记特性(mark trait), 它本身没有功能

    Unpin 通知编辑器,给定类型不需要维持关于 "这个值是否可以安全移动" 的任何保证

    就像Send 和Sync一样，编辑器会自动为所有可以证明安全的类型实现Unpin // 自由使用Pin

    !Unpin

    表示法 impl !Unpin for SomeType
    当一个指向该类型的指针被包裹在Pin中使用时，这个SomeType 类型就必须维持这些保证(不被移动的保证)才能确保使用时的内存安全

    Stream trait 
    Iterator 序列，提供了next() 方法
    Future 中，随时间就绪的概念
        提供了 poll()放啊

    Stream trait 就是同时可以拥有 next() poll() 方法
*/

use std::pin::Pin;
use std::task::{Context,Poll};

trait Stream {
    type Item;

    fn poll_next(
        self:Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<Option<Self::Item>>;
}



// 使用异步任务trpl::spawn_task()构建Stream 
// 任务是异步操作集合的边界;任务之间及任务内部都可并发运行；多个future 之间切换
fn get_intervals() -> impl Stream<Item = u32> {
    let (tx,rx) = trpl::channel();

    trpl::spawn_task(async move{
        let mut count = 0;
        loop{
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;

            if let Err(send_error) = tx.send(count) {
                eprintln!("Could not send invterval {count}: {send_error}");
                break;
            };
        }
    });

    ReceiverStream::new(rx)
}
// 使用线程thread来构建Stream // 上百万任务会消耗光
// 线程是同步操作集合的边界;线程之间可以并发运行
fn get_intervals() -> impl Stream<Item = u32> {
    let (tx,rx) = trpl::channel();
    // trpl::spawn_task(async move{
    thread::spawn( move||{
        let mut count = 0;
        loop{
            // trpl::sleep(Duration::from_millis(1)).await;
            thread::sleep(Duration::from_millis(1));
            count += 1;

            if let Err(send_error) = tx.send(count) {
                eprintln!("Could not send invterval {count}: {send_error}");
                break;
            };
        }
    });

    ReceiverStream::new(rx)
}

// 运行时(executor) 负责管理任务,任务负责管理futures

// 线程 与 异步 混合
use std::{thread,time::Duration};

fn main() {
    let (tx,mut rx) = trpl::channel();

    // 线程并行 send()
    thread::spawn( move||{
        for i in 1..11{
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    // 异步并发 rev() 接收
    trpl::run(async {
        while let Some(message) = rx.recv().await {
            println!("{message}");
        }
    });
}
