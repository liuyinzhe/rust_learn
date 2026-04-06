
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener
};

/*
tokio：一个异步运行时库，提供事件循环、任务调度、异步 I/O 等功能。
TcpListener：来自 tokio::net，用于监听 TCP 连接。
BufReader：带缓冲的读取器，可以减少系统调用，提高读取效率。
AsyncBufReadExt：为实现了 AsyncBufRead 的类型（如 BufReader）提供的扩展 trait，包含 read_line 等异步方法。
AsyncWriteExt：为实现了 AsyncWrite 的类型（如 TCP 流）提供的扩展 trait，包含 write_all 等异步方法
*/


// cargo run --example example7
#[tokio::main] // 多线程写法
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    // socket 客户端的流Stream, _address 客户端地址
    // 无限循环,接受连接
    loop {
        let (mut socket, _address) = listener.accept().await.unwrap();
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
                let bytes_read = reader.read_line(&mut message).await.unwrap(); // 读取一行消息,传给 message
                if bytes_read == 0 { // 表示连接已关闭（对端关闭写端或断开连接），此时跳出循环
                    break;
                    // 客户端关闭连接,跳出循环
                }
                stream_writer.write_all(message.as_bytes()).await.unwrap(); // 回显消息
                message.clear(); // 清空消息缓冲区
            }
        });
    }
}
