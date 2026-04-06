
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::TcpListener, signal, sync::broadcast // 广播,用于任务之间通信
};
use tokio_util::sync::CancellationToken;

// cargo add tokio-util
// cargo add tracing
// cargo add tracing-subscriber

// cargo run --example example8 // demo 3/4/5 chat server
#[tokio::main] // 多线程写法
async fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).unwrap(); // 设置全局默认订阅者


    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    // socket 客户端的流Stream, _address 客户端地址

    // 最多存储10个客户端信息,每个客户端信息是一个字符串
    // let (tx, _) = broadcast::channel::<String>(10);
    let (tx, _) = broadcast::channel(10); // 自动推理类型
    let token = CancellationToken::new();
    let cancel_token = token.clone();
    tokio::spawn(async move {
        match signal::ctrl_c().await {
            Ok(_) => {
                tracing::warn!("Shutdown Tasks");
                cancel_token.cancel(); // 所有token 停止执行
            }
            Err(err) => {
                tracing::error!("Error: {err:#?}" );
            }
        }
    });
    // 无限循环,接受连接
    loop {
        let token = token.clone();
        let tx = tx.clone();
        let mut rx = tx.subscribe(); // 创建接收端
        let (mut socket, address) = listener.accept().await.unwrap();
        // listener 没有copy 不是clone ,所以不能在异步任务中使用,会被 move 无法重复使用
        tokio::spawn(async move { // 多个客户端异步任务
            // 将 TcpStream 拆分为一个只读句柄和一个只写句柄
            let (stream_reader, mut stream_writer) = socket.split();

            // 中转缓冲区，用于存储读取到的消息
            let mut message = String::new();
            // BufReader::new(stream_reader)：创建一个带内部缓冲区的读取器，包装 stream_reader
            // mut reader：需要可变，因为读取会消耗缓冲区内容并更新内部位置。
            let mut reader = BufReader::new(stream_reader);

            // 无限循环回显
            loop {
                tokio::select! { // 等待多个并发的分支,直到有一个完成,就执行对应的代码
                    result = reader.read_line(&mut message) => {
                        tracing::info!("Received message from client: {}",&message);
                        if result.unwrap() == 0 {
                            break;
                        }
                        tx.send((message.clone(),address)).unwrap();
                        message.clear(); // 清空消息缓冲区
                    }
                    result = rx.recv() => {
                        let (received_message, sender_address) = result.unwrap();
                        if address != sender_address {
                            tracing::info!("Received message from client: {}",&received_message);
                            stream_writer.write_all(received_message.as_bytes()).await.unwrap(); // 回显消息    
                        }
                        
                    }
                    _ = token.cancelled() => {
                        tracing::info!("Cleaning up ...");
                        return;
                    }

                }
            }
        });
    }
}
