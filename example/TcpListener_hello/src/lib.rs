// use std::thread::JoinHandle;
use  std::{sync::mpsc, thread};
use std::sync::{Arc,Mutex};
pub struct ThreadPool{
    //threads: Vec<JoinHandle<()>>,
    workers: Vec<Worker>, // 工作线程集合,类型Worker
    sender: Option<mpsc::Sender<Job>> // 任务发送通道(Option用于安全析构)
}
// 类型别名简化
type Job = Box<dyn FnOnce() + Send + 'static>;
/*
FnOnce()：可调用一次的任务闭包
Send：可跨线程传递
'static：闭包捕获的数据具有静态生命周期
*/

/*
Fn、FnMut 和 FnOnce 是 Rust 标准库中定义的三个特征，用于描述闭包(或任何可调用对象)的行为。
它们的主要区别在于如何访问捕获的变量以及在被调用时的所有权规则：

FnOnce：表示闭包可以被调用一次。被调用后，闭包本身会被消耗，不能再被使用。
FnMut：表示闭包可以被多次调用，并且在被调用时可以修改捕获的变量。
Fn：表示闭包可以被多次调用，并且只读取捕获的变量而不修改它们。
*/

impl ThreadPool {
    pub fn new(size:usize) -> ThreadPool{
        assert!(size>0);

        let (sender,receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver)); // Arc 原子应用计数,共享接收端的所有权，Mutex互斥锁,确保同一时间自由一个worker能获得所有权

        let mut  workers = Vec::with_capacity(size);
        //Vec::with_capacity只会分配内存空间，但是不会填充值；
        //vec!会通过复制初始值的方式来填充值；
        
        for id in 0..size{
            workers.push(Worker::new(id,Arc::clone(&receiver))); // receiver 不能多次移动所有权，使用Arc指针
        }
        ThreadPool{ workers ,sender:Some(sender)} // 返回 结构体
    }

    pub fn execute<F>(&self,f:F)
    where // 引入泛型约束子句
        F: FnOnce() + Send + 'static,// 参数 泛型的trait 特性绑定
        {
            let job = Box::new(f);

            // self.sender: 访问线程池的发送通道
            // .as_ref(): 将 Option<Sender> 转为 Option<&Sender>
            // 避免移动所有权(因 &self 是不可变引用)
            // .unwrap(): 解包 Option(确信此时 sender 是 Some 状态)
            // .send(job): 关键操作 - 通过通道发送任务
                // 将 job(装箱的任务)送入任务队列
                // 工作线程将从另一端接收
            //  unwrap(): 处理发送结果(失败时 panic)
            self.sender.as_ref().unwrap().send(job).unwrap();
        }
}

impl Drop for ThreadPool {

    fn drop(&mut self) {
        drop(self.sender.take()); // 关闭发送端,receiver 也会对自动关闭

        /*
            drain(..) 是 Vec 的方法
            移除并返回向量中所有元素的迭代器
            .. 表示整个范围(从第一个到最后一个元素)
        */
        for worker in &mut self.workers.drain(..){
            println!("Shutting down work {}", worker.id);

            /* 
            worker.thread：
                访问 Worker 结构体的 thread 字段(JoinHandle<()> 类型)
            join()：
                阻塞当前线程(主线程)直到目标线程完成执行
                返回 Result<(), Box<dyn Any + Send>>
            */
            worker.thread.join().unwrap();// 需要所有权
        }
    }

}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,

}
impl Worker {
    fn new(id:usize,receiver:Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move|| loop {
            let message = receiver.lock().unwrap().recv();
            // match 错误
            match message {
                Ok(job) => {
                    println!("Worker {id} get a job; executing.");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
            
            // lock().unwrap() 互斥锁 recv().unwrap() 接收任务
            
            // .unwrap() 简单处理recv()错误
            // let job = receiver.lock().unwrap().recv().unwrap();
            // println!("Worker {} get a job; executing",id);
            // job();
        }) ;

        Worker {id, thread}

    }
}
