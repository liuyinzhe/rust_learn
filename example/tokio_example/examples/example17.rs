// cargo add rayon
// cargo add tokio -F full

/*
    select!()
    等待多个 Future 中第一个完成的那个, 并且会自动取消其他的future
    // 并发查询任务,查到了，可以停止其他任务
*/

// cargo run --example example17

use std::time::Duration;

use tokio::{sync::{mpsc,broadcast}, time::sleep};
// mpsc 多生产者单消费者通道（Multi-Producer, Single-Consumer）
// broadcast 广播通道，允许多个发送者和多个接收者，每个接收者都能收到所有发送的值

async fn _do_work(){
    sleep(Duration::from_secs(3)).await;
}

async fn _timeout(secs: f32){
    sleep(Duration::from_secs_f32(secs)).await;
}


async fn receiver (
    mut rx: mpsc::Receiver<u32>,
    mut broadcast_re: broadcast::Receiver<u32>,
){
    loop {
        tokio::select! { // 竞速语义，先完成的future 会执行，其他future 会被取消
            Some(n) = rx.recv() => println!("从 mpsc channel 收到信息 {n}"),
            Ok(n) = broadcast_re.recv() => println!("从 broadcast channel 收到信息 {n}"),
        }
    }
}

#[tokio::main]
async fn main() {

    /*
        select!()的语法类似于 match, 但多了一步。格式是
        (接收值) = (future) => (当它先完成时执行的代码)
        // future 任务
    */
    // tokio::select! {
    //     _ = do_work() => println!("do_work() completed first"),
    //     _ = timeout(1.0) => println!("timeout() completed first"),
    // }

    let (tx,rx) = mpsc::channel::<u32>(1);
    let (broadcast_tx,broadcast_rx) = broadcast::channel::<u32>(1);
    // let tx_b = broadcast_tx.clone();
    // let mut rx_b = tx_b.subscribe();
    // let _ = rx_b.recv().await.unwrap();
    // drop(rx_b);
    tokio::spawn(receiver(rx,broadcast_rx));

    for c in 0..10 {
        if c %2 == 0 {
            tx.send(c).await.unwrap(); // send recv
        }else{
            broadcast_tx.send(c).unwrap(); // send recv
        }
        sleep(Duration::from_secs(1)).await;
    }


}