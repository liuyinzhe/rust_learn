// cargo add rayon
// cargo add tokio -F full

/*
    广播通道
    Broadcast Channel

    Tokio 提供了一种标准rust中没有的通道类型:
    广播通道(broadcast channel)
    是一种多生产者、多消费者的广播队列
      也非常适合: 单发送者 + 多接收者 的广播场景
    每一条被发送的消息，都会被所有消费者接收到

    工作机制

    但一条数据被发送
    所有Receiver都会接收到通知
    数据在通道中只存储一份
    每个Receiver在接收时会获得该数据的一个克隆副本
    当所有接收者都完成接收后,该数据才会从通道中释放

    慢接收者问题
    Lagging

    由于消息必须保留知道所有接收者都处理完，因为broadcast 通道天然存在
    "慢接收者问题":
    如果某个接收者处理速度很慢，其它接收者已经读完了，但这条消息仍不能
    释放，导致通道逐渐被填满
    
    解决机制: 硬容量限制
        满了之后最旧的消息就会被丢弃

    关闭
    Closing

    当所有Sender 都被丢弃时，通道关闭
    接收者读完所有剩余数据后，下次recv()返回:
    RecvError::Closed

    当某个Receiver 被丢弃:
    该接收者未读的消息会被标记为"已读"
    如果它是最后一个未读该消息的接收者，则该消息立即从通道中删除
*/

// cargo run --example example15

#[tokio::main]
async fn main() {
    // 通道初始化
    let (tx, mut rx) =
        tokio::sync::broadcast::channel::<String>(16);

    for n in 0..20 { // 订阅接收者有20个
        let mut messages= tx.subscribe();
        // tx.subscribe()：从发送端创建一个新的广播接收者 Receiver<String>。该方法返回一个新的 Receiver 实例
        // tx.subscribe() 会克隆内部的通道状态

        tokio::spawn(async move {
            // messages.recv().await：异步等待接收一条消息
            while let Ok(msg) = messages.recv().await {
                println!("{n}:{msg}");
            }
        });
    }
    // 单发送 广播内容
    tx.send("Hello channel".to_string()).unwrap();
    drop(tx);// 丢弃原始 Sender 实例 tx 变量

    // 原始通道 接收者 Receiver<String> rx 变量
    while let Ok(msg) = rx.recv().await {
        println!("Main: {msg}");
        
    }
}