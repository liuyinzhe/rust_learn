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
    // 设置日志记录器
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).unwrap(); // 设置全局默认订阅者

    // 绑定 TCP 监听器
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    // 最多存储10个客户端信息,每个客户端信息是一个字符串
    // let (tx, _) = broadcast::channel::<String>(10);
    // 建立广播 // 多生产者 - 多消费者
    // tx 是发送端,可以克隆多个发送端,每个发送端都可以发送消息
    let (tx, _) = broadcast::channel(10); // 自动推理类型

    // 创建 取消令牌
    /*
    所有克隆出来的令牌（包括 token、cancel_token、以及每个客户端任务持有的 token）都共享同一个内部状态。
    只要任意一个令牌调用了 .cancel()，所有令牌的 .cancelled() future 都会立即完成。
    */
    let token = CancellationToken::new();
    let cancel_token = token.clone(); // 克隆一个取消令牌,用于在异步任务中使用

    // spawn 异步任务结构域
    tokio::spawn(async move { // 移动cancel_token 所有全
        // 处理 Ctrl+C 信号
        match signal::ctrl_c().await { // 异步等待 Ctrl+C 信号
            Ok(_) => {
                tracing::warn!("Shutdown Tasks");
                cancel_token.cancel(); // 所有token 停止执行,token.cancelled() == true
            }
            Err(err) => {
                tracing::error!("Error: {err:#?}" );
            }
        }
    });
    // 无限循环,接受连接, 监听listener
    loop {
        let token = token.clone();
        let tx = tx.clone();
        let mut rx = tx.subscribe(); // 创建接收端
        // socket 是 TcpStream, address 是 SocketAddr
        let (mut socket, address) = listener.accept().await.unwrap(); // await 异步,每当有新的连接时,就创建一个新的异步任务来处理该连接
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
            loop {// 同时等待多个异步操作，哪个先完成就执行对应的分支，其余分支被取消。这是实现高并发、非阻塞事件循环
                tokio::select! { // 等待多个并发的分支,直到有一个完成,就执行对应的代码
                    // 读取客户端消息
                    result = reader.read_line(&mut message) => {
                        tracing::info!("Received message from client: {}",&message);
                        if result.unwrap() == 0 { // 连接关闭,没有读取到数据,返回0
                            break;
                        }
                        tx.send((message.clone(),address)).unwrap(); // 通过广播发送端发送(message.clone(),address)给所有客户端。
                        // 消息中包含发送者地址，以便接收方判断是否是自己
                        message.clear(); // 清空消息缓冲区
                    }
                    // 接收其他客户端的消息
                    result = rx.recv() => {
                        let (received_message, sender_address) = result.unwrap();
                        // 判断是否是自己,不是自己的则回显,发送给其他人,但是自己不回显
                        if address != sender_address {
                            tracing::info!("Received message from client: {}",&received_message);
                            stream_writer.write_all(received_message.as_bytes()).await.unwrap(); // 回显消息    
                        }
                        
                    }
                    // 监听取消令牌,如果被取消了,就退出循环
                    _ = token.cancelled() => {
                        tracing::info!("Cleaning up ...");
                        return;
                    }

                }
            }
        });
    }
}