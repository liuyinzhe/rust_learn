
use std::{
    fs, io::{prelude::*,BufReader}, net::{TcpListener, TcpStream},thread
};
use hello::ThreadPool;

fn main() {
    // 绑定监听
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // 建立 线程池[对象:[workers列表,sender通信]]
    let pool =ThreadPool::new(4);
    // 只处理2个连接(take(2)演示用)
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        // handle_connection(stream);
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    

    // buf_reader.lines() 返回一个迭代器，产生 io::Result<String> 类型即 (Result<String, std::io::Error>)
    // .next() 获取迭代器的第一个元素，返回类型是 Option<io::Result<String>>
    // Option<(io::Result<String, std::io::Error>>
    // 第一个处理 Option  Some,None
    // 第二个处理 Result  String,std::io::Error
    let request_line= buf_reader.lines().next().unwrap().unwrap();

    // if request_line.starts_with("GET / HTTP/1.1") { // http://localhost:7878

    //     let status_line = "HTTP/1.1 200 OK";
    //     let contents = fs::read_to_string("hello.html").unwrap();
    //     let length = contents.len();

    //     let response = 
    //         format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
        
    //     // println!("Request : {http_request:#?}");
    //     // let response = "HTTP/1.1 200 OK\r\n\r\r";
    //     stream.write_all(response.as_bytes()).unwrap();

    // }else{ // http://localhost:7878/123

    //     let status_line = "HTTP/1.1 404 NOT FOUND";
    //     let contents = fs::read_to_string("404.html").unwrap();
    //     let length = contents.len();

    //     let response = format!(
    //         "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
        
    //     stream.write_all(response.as_bytes()).unwrap();

    // }
    
    let (status_line,filename) = if request_line.starts_with("GET / HTTP/1.1") {
        ("HTTP/1.1 200 OK","hello.html")
    }else{
        ("HTTP/1.1 404 NOT FOUND","404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
    stream.write_all(response.as_bytes()).unwrap();

}
// http://localhost:7878

