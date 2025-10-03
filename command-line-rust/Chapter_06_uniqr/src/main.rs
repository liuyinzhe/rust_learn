// cargo add  clap --features derive
// cargo add --dev anyhow assert_cmd predicates pretty_assertions
// cargo add --dev rand
// cargo add --dev tempfile

// cargo add anyhow


use anyhow::{anyhow, Result};
use clap::Parser;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `uniq`
struct Args {
    /// Input file
    #[arg(value_name = "IN_FILE", default_value = "-")]
    in_file: String,

    /// Output file
    #[arg(value_name = "OUT_FILE")]
    out_file: Option<String>,

    /// Show counts
    #[arg(short, long)]
    count: bool,
}

// --------------------------------------------------
fn main() {
    if let Err(e) = run(Args::parse()) { // 执行,并错误捕获
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

// --------------------------------------------------
// 主要的业务逻辑函数,返回 anyhow::Result<()>
// anyhow::Result 是 std::result::Result<T, anyhow::Error> 的别名
fn run(args: Args) -> Result<()> {
    // 打开输入文件
    // map_err 用于转换错误类型,将 open 返回的错误包装为 anyhow::Error
    let mut file =
        open(&args.in_file).map_err(|e| anyhow!("{}: {e}", args.in_file))?;
    
    // 动态分发输出目标
    // Box<dyn Write> 是一个 trait object,可以在运行时决定具体类型
    let mut out_file: Box<dyn Write> = match &args.out_file { // 对于未知大小的变量,动态存入堆,内存地址包装为Box智能指针,存储到栈
        Some(out_name) => Box::new(File::create(out_name)?),
        _ => Box::new(io::stdout()), // 其它情况(获取args.out_file失败),输出到STDOUT
    };


    /*
    闭包需要是 mut 的原因在于闭包捕获了 out_file,并且在对 out_file 进行 write! 操作时,需要可变借用。
    因为 write! 操作会改变 out_file 的内部状态(例如,写入缓冲区、修改文件指针等)。

    在 Rust 中,如果一个闭包捕获了可变引用或者以可变方式捕获了变量,那么该闭包必须标记为 mut。
    因为调用闭包时,可能会修改其捕获的变量,所以调用闭包被视为一种可变操作。
    */
    // 定义一个闭包(closure)来处理输出
    // 闭包可以捕获外部变量(args.count, out_file)
    let mut print = |num: u64, text: &str| -> Result<()> {
        if num > 0 {            // 重复计数大于0
            if args.count {
                // {num:>4} 表示右对齐, 宽度为4的格式化
                write!(out_file, "{num:>4} {text}")?;
            } else {
                write!(out_file, "{text}")?;
            }
        };
        Ok(())
    };

    // 初始化变量
    let mut line = String::new();       // 当前读取的行
    let mut previous = String::new();   // 前一行内容
    let mut count: u64 = 0;             // 重复计数

    // 主循环: 按照每一行读取并处理
    loop {
        // read_line 返回读取的字节数,0 表示 EOF
        let bytes = file.read_line(&mut line)?; // read_line 返回长度, 每行内容复制为 line
        if bytes == 0 {
            break; // EOF 中断循环
        }

        if line.trim_end() != previous.trim_end() { // String.trim_end()去除行尾空白
            print(count, &previous)?;
            previous = line.clone(); // clone 保存当前行给 previous
            count = 0;
        }

        count += 1; // 类加
        line.clear(); // 清空 line 当前行缓存变量
    }
    print(count, &previous)?; // 补充最后一行的逻辑判断输出

    Ok(())
}

// --------------------------------------------------
// 打开文件的辅助函数，返回一个实现了 BufRead trait 的对象
fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
