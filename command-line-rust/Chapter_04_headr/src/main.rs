use anyhow::Result; // 错误处理库,简化错误处理
use clap::Parser;   // 命令行参数解析库
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read}; // 输入输出相关

// cargo add  clap --features derive
// cargo add  anyhow

#[derive(Parser, Debug)]  // 使用 clap 的派生宏自动实现命令行解析
#[command(author, version, about)]  // 自动从 Cargo.toml 获取元信息
/// Rust version of `head`
struct Args {
    /// Input file(s) // 参数文档注释
    #[arg(default_value = "-", value_name = "FILE")] // 默认值"-"表示标准输入
    files: Vec<String>,  // 文件列表,支持多个文件

    /// Number of lines         // 显示行数
    #[arg(
        short('n'),             // 短选项 -n
        long,                   // 长选项 --lines
        default_value = "10",   // 默认显示10行
        value_name = "LINES",   // 值名称
        value_parser = clap::value_parser!(u64).range(1..) // 验证器,必须是≥1的整数
    )]
    lines: u64,     // args.lines 调用名称

    /// Number of bytes         // 显示字节数
    #[arg(
        short('c'),             // 短选项 -c
        long,                   // 长选项 --bytes  
        value_name = "BYTES",
        conflicts_with("lines"),// 与 lines 参数互斥
        value_parser = clap::value_parser!(u64).range(1..) // 验证器,必须是≥1的整数
    )]
    bytes: Option<u64>,// args.bytes 调用名称
}

// --------------------------------------------------
fn main() {
    // 解析参数并运行,如果出错则打印错误并退出
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

// --------------------------------------------------
fn run(args: Args) -> Result<()> {
    let num_files = args.files.len(); // 文件数量
    // 遍历Vec向量
    for (file_num, filename) in args.files.iter().enumerate() {
        match open(filename) {  // 尝试打开文件
            Err(err) => eprintln!("{filename}: {err}"),  // 打开失败打印错误
            Ok(mut file) => {
                // 如果多个文件,显示文件名标题
                if num_files > 1 {
                    println!(
                        "{}==> {filename} <==",
                        if file_num > 0 { "\n" } else { "" }, // 非第一个文件(file_num -> index)前加空行
                    );
                }
                // 按字节模式或行模式处理
                if let Some(num_bytes) = args.bytes { // 捕获参数 存入 num_bytes

                    /*
                        file 是一个实现了 BufRead trait 的对象(通过 open 函数返回的 Box<dyn BufRead>)。
                        BufRead 提供了一个 read_line 方法。

                        read_line 方法的作用是: 从输入流中读取一行数据(直到遇到换行符,即 b'\n'),并将读取的数据追加到指定的字符串中。
                        注意: 它会包括换行符(如果遇到换行符的话)。

                        参数 &mut line: 这是一个可变的字符串引用。read_line 会将读取的内容追加到这个字符串中。
                        因此,在每次调用前,我们使用 line.clear() 清空字符串,以免上一行的内容被保留。

                        返回值: read_line 返回一个 Result<usize>,即读取的字节数(包括换行符)。如果遇到文件末尾(EOF),则返回 Ok(0)。

                        ? 操作符: 如果 read_line 返回一个 Err,则整个函数会提前返回这个错误(利用 anyhow::Result 进行错误传播)。
                        如果返回 Ok(n),则 n 是读取的字节数,赋值给 bytes。

                        随后检查 bytes == 0,如果为0,表示已经到达文件末尾,循环提前退出。

                        然后打印这一行(包括换行符,因为 read_line 读取的内容包含换行符)。

                        最后,清空字符串 line,为下一行读取做准备。
                    */
                    let mut buffer = vec![0; num_bytes as usize]; // 使用 vec! 宏创建向量 // vec![向量的初始值; 向量的长度]
                    /*
                    // 假设 buffer = [0, 0, 0, 0, 0] (5个字节)
                    // 假设 bytes_read = 3 (实际只读取了3个字节)
                    &buffer[..bytes_read]  // 取前3个字节: [0, 0, 0]
                    */
                    let bytes_read = file.read(&mut buffer)?;  // 读取buffer提取的数据
                    print!(
                        "{}",
                        String::from_utf8_lossy(&buffer[..bytes_read])  // UTF-8 编码//替换无效序列为� // 将字节转换为字符串打印(支持非UTF-8字符)
                    );
                } else {
                    let mut line = String::new();
                    for _ in 0..args.lines {
                        // read_line 读取行内容保存到缓存 line 变量,Result<usize> 使用? 解读提取数值
                        let bytes = file.read_line(&mut line)?; // 保留原始换行符;read_line 只读取一行,包含CR/LF,返回字节大小,遇到EOF(End Of File),则返回0
                        /*
                        如果用BufRead::lines 则不会保留换行符(0xA字节)或者CRLF(0xD,0xA)字节 

                        ASCII值为13,0xD(CR,Carriage Return) \r
                        ASCII值为10,0xA(LF,Line Feed) \n
                        */
                        if bytes == 0 { // 读到文件末尾
                            break;
                        }
                        print!("{line}");
                        line.clear();// 清空缓冲区供下次使用
                    }
                }
            }
        }
    }

    Ok(())
}

// --------------------------------------------------
fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))), // "-" 表示标准输入
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))), // 其他为普通文件
    }
}
