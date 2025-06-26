
use std::thread::JoinHandle;
use  std::{sync::mpsc, thread};
use std::sync::{Arc,Mutex};
pub struct ThreadPool{
    //threads: Vec<JoinHandle<()>>,
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>
}
// 类型别名简化
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size:usize) -> ThreadPool{
        assert!(size>0);

        let (sender,receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver)); // Arc 原子应用计数,共享接收端的所有权，Mutex控制次数，确保同一时间自由一个worker能获得所有权

        let mut  workers = Vec::with_capacity(size);
        for id in 0..size{
            workers.push(Worker::new(id,Arc::clone(&receiver))); // receiver 不能多次移动所有权，使用Arc指针
        }
        ThreadPool{ workers ,sender:Some(sender)} // 返回 结构体
    }

    pub fn execute<F>(&self,f:F)
    where 
        F: FnOnce() + Send + 'static,// 参数 泛型的trait 特性绑定
        {
            let job = Box::new(f);
            self.sender.as_ref().unwrap().send(job).unwrap();
        }
}

impl Drop for ThreadPool {

    fn drop(&mut self) {
        drop(self.sender.take()); // 关闭发送端
        for worker in &mut self.workers.drain(..){
            println!("Shutting down work {}", worker.id);

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
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} get a job; executing",id);
            job();
        }) ;

        Worker {id, thread}

    }
}