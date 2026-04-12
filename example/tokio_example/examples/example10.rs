
// cargo add tokio-util
// cargo add tracing
// cargo add tracing-subscriber

// cargo run --example example10 // 错误处理(12)

use std::{fs, net::TcpStream, num::ParseIntError, path::Path};


fn main() {
    let address = "127:80";
    let connection = TcpStream::connect(address);
    
    match connection {
        Ok(stream) => {
            println!("成功链接 {:?}", stream);
        }
        // Err(e) => {
        //     println!("Error: {}", e);
        // }
        Err(e) => match e.kind() {
            std::io::ErrorKind::ConnectionRefused => println!("连接被拒绝"),
            std::io::ErrorKind::NetworkUnreachable => println!("网络不可达"),
            _ => println!("Error: {}", e),
        }

    }

    match try_connect_to_server("127:80") {
        Ok(stream) => {
            println!("Success {stream:#?}");
        }
        Err(e) => {
            println!("Error: {e:#?}");
        }
    }

    let _ = connect_and_validate(address); 
    if let Ok(stream) =  connect_and_validate(address){
        println!("Success {stream:#?}");
    }


}

fn try_connect_to_server(address: &str) -> Result<TcpStream, std::io::Error> {
    TcpStream::connect(address)
}


fn connect_and_validate(address: &str) -> Result<TcpStream, std::io::Error> {
    let stream = TcpStream::connect(address)?;
    Ok(stream)
}

fn parse_address(content: &str) -> Result<(String, u16), ParseIntError> {
    let parts:Vec<&str> = content.split(":").collect();
    let host = parts[0].to_string();
    let port = parts[1].parse::<u16>()?;
    Ok((host, port))
}
// type GenericResult<T> = Result<T, Box<dyn std::error::Error>>;
// fn load_config_and_connect() -> Result<TcpStream, Box<dyn std::error::Error>> {
// fn load_config_and_connect() -> GenericResult<TcpStream> {
fn load_config_and_connect() -> anyhow::Result<TcpStream> { // anyhow box 包装器
    let config_file = Path::new("server.txt");
    let raw_text = fs::read_to_string(config_file)?;
    let (host, port) = parse_address(raw_text.trim())?;
    let address = format!("{host}:{port}");
    let stream = TcpStream::connect(&address)?;
    Ok(stream)

}


fn anyhow_load_config_and_connect() -> anyhow::Result<TcpStream> {
    let config  = Path::new("server.txt");
    let raw_text = fs::read_to_string(config)?;
    let address = raw_text.trim();

    if address.is_empty() {
        // 报错返回信息
        anyhow::bail!("Server address cannot be empty");
    }

    if !address.contains(":") {
        // 返回错误信息
        return Err(anyhow::Error::msg("Server address must contain a colon"));
    }

    let stream = TcpStream::connect(address)?;
    Ok(stream)
}
