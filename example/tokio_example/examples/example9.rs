use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::TcpListener, signal, sync::broadcast // 广播,用于任务之间通信
};
use tokio_util::sync::CancellationToken;
use tokio::task::JoinSet;   // 管理多个任务

// cargo add tokio-util
// cargo add tracing
// cargo add tracing-subscriber

// cargo run --example example9 // demo 3/4/5 chat server 优化版本

#[tokio::main] // 多线程写法
async fn main() {
    // 设置日志记录器
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).unwrap(); // 设置全局默认订阅者

    // 绑定 TCP 监听器
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    tracing::info!("Server is running on 127.0.0.1:8080");

    // 最多存储10个客户端信息
    // 建立广播 // 多生产者 - 多消费者
    // tx 是发送端,可以克隆多个发送端,每个发送端都可以发送消息
    // rx 是接收端,可以订阅多个接收端,每个接收端都可以接收发送端发送的消息
    let (tx, _) = broadcast::channel(10);

    // 创建 取消令牌
    /*
    所有克隆出来的令牌（包括 main_token、cancel_token、以及每个客户端任务持有的 token）都共享同一个内部状态。
    只要任意一个令牌调用了 .cancel()，所有令牌的 .cancelled() future 都会立即完成。
    */
    let token = CancellationToken::new();
    let cancel_token = token.clone();

    // 1. 启动 Ctrl+C 监听任务
    tokio::spawn(async move { // spawn 异步任务结构域
        match signal::ctrl_c().await { // 异步等待 Ctrl+C 信号
            Ok(_) => {
                tracing::warn!("Shutdown Tasks");
                cancel_token.cancel(); // 取消所有任务,token.cancelled() == true
            }
            Err(err) => tracing::error!("Error: {err:#?}"),
        }
    });

    // 2. 创建一个任务集，用于管理所有客户端任务
    let mut join_set = JoinSet::new();

    // 3. 主循环：同时等待 accept 和取消信号
    let main_token = token.clone();
    // // 无限循环回显客户端消息
    loop {
        tokio::select! { // 等待多个并发的异步分支,有一个完成进入loop,就执行对应的代码,并退出当前循环,然后重新循环
            accept_result = listener.accept() => {
                match accept_result { // 分支1：接受新连接任务
                    Ok((socket, address)) => {
                        let token = token.clone(); // 克隆取消令牌,取消令牌之间关联;结构体内容是Arc指针
                        let tx = tx.clone();
                        let rx = tx.subscribe(); // 订阅广播通道,每个客户端任务都有自己的接收端
                        // 将任务句柄加入 JoinSet
                        join_set.spawn(async move {
                            handle_client(socket, address, token, tx, rx).await;
                        });
                    }
                    Err(e) => {
                        tracing::error!("Accept error: {e}");
                        break;
                    }
                }
            }
            _ = main_token.cancelled() => { // 分支2：取消信号任务
                tracing::warn!("Cancellation received, stopping accept loop");
                break;  // 退出主循环，不再接受新连接
            }
        }
    }

    // 4. 可选：显式关闭监听器，让操作系统尽快释放端口
    drop(listener);

    // 5. 等待所有已连接的客户端任务完成
    tracing::info!("Waiting for {} client tasks to finish...", join_set.len());
    // join_set.join_next() 异步等待任意一个任务完成  Some(Result<JoinHandle::Output, JoinError>)
    while let Some(res) = join_set.join_next().await {
        if let Err(e) = res {
            tracing::error!("A client task panicked: {e}");
        }
    }
    tracing::info!("All clients disconnected, server exiting.");
}

// 将原客户端任务逻辑抽取为独立函数，便于阅读
async fn handle_client(
    mut socket: tokio::net::TcpStream,
    address: std::net::SocketAddr,
    token: CancellationToken,
    tx: broadcast::Sender<(String, std::net::SocketAddr)>,
    mut rx: broadcast::Receiver<(String, std::net::SocketAddr)>,
) {
    // 将 TcpStream 拆分为一个只读句柄和一个只写句柄
    let (stream_reader, mut stream_writer) = socket.split();
    // BufReader::new(stream_reader)：创建一个带内部缓冲区的读取器，包装 stream_reader
    // mut reader：需要可变，因为读取会消耗缓冲区内容并更新内部位置。
    let mut reader = BufReader::new(stream_reader);
    
    // 中转缓冲区，用于存储读取到的消息
    let mut message = String::new();

    loop { // 无限循环回显
        // 等待多个并发的异步分支,有一个完成进入loop,就执行对应的代码,并退出当前循环,然后重新循环
        tokio::select! {
            result = reader.read_line(&mut message) => { // 分支 1：读取客户端消息
                if result.unwrap() == 0 { break; }
                tracing::info!("Received from {}: {}", address, message);
                // 通过广播发送端发送(message.clone(),address)给所有客户端。
                let _ = tx.send((message.clone(), address));
                message.clear();
            }
            result = rx.recv() => { // 分支 2：接收广播消息
                match result {
                    Ok((received, sender_addr)) => {
                        // 判断是否是自己,不是自己的则回显,发送给其他人,但是自己不回显
                        if address != sender_addr {
                            tracing::info!("Broadcasting to {}: {}", address, received);
                            // 回显消息
                            let _ = stream_writer.write_all(received.as_bytes()).await;
                        }
                    }
                    //同时任务容量超过10个,导致丢失, n 表示丢失的消息数量
                    Err(broadcast::error::RecvError::Lagged(n)) => {
                        tracing::warn!("Client {} lagged behind by {} messages", address, n);
                    }
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
            // 监听取消令牌,如果被取消了,就退出循环
            _ = token.cancelled() => {
                tracing::info!("Client {} shutting down", address);
                break;
            }
        }
    }
}