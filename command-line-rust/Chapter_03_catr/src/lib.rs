use std::error::Error;
use clap::Parser;
use std::fs::File;
use std::io::{self,BufRead,BufReader};
use clap::{Command,Arg,ArgAction};

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> { // 返回值为错误信息，或者一个实现了BufRead特性的值
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))), // 如果输入的参数filename 是 "-",则从STDIN 读取
        _ => Ok(Box::new(BufReader::new( // 其它情况
            File::open(filename)? // ?解Result 返回值,获取value
        ))),
    }
}


type MyResult<T> = Result<T,Box<dyn Error>>;

// pub fn run() -> MyResult<()> { // pub 公共函数,提供外部访问
//     println!("Hello, world");
//     Ok(())
// }

pub fn run(config: Config) -> MyResult<()> {
    // dbg!(config);
    //  cargo run -- tests/inputs/*.txt
    for filename in config.files {
        // println!("{}",filename);
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}",filename,err), // 捕获错误到err 变量中,并将错误信息输出到STDERR 中
            Ok(file) => {
                println!("Opend {}", filename);
                let mut last_num = 0;
                for (line_num, line) in file.lines().enumerate() { // 按行迭代
                    let line = line?;
                    if config.number_lines {
                        println!("{:>6}\t{}",line_num+1, line);
                    // }else{
                    //     println!("{}", line);
                    // }
                    }else if config.number_nonblank_lines{
                        if !line.is_empty() { // 非空行
                            last_num += 1;
                            println!("{:>6}\t{}",line_num, line);
                        }else{
                            println!(); // 空行就输出回车
                        }
                        
                    }else{
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

#[derive(Debug)] // 添加 Debug 特性,方便打印结构体,println!("{:?}",x)
pub struct Config { // 结构体
    files: Vec<String>, // 向量
    number_lines: bool,  // 是否打印行数
    number_nonblank_lines:bool, // 是否打印非空白行的行号
}

#[derive(Debug, Parser)]
// 为命令行应用添加元数据：作者、版本和描述
// 这些信息会自动从 Cargo.toml 中获取
#[command(author, version, about)]
struct Args {
    /// Input file(s)
    #[arg(value_name = "FILE", default_value = "-")] // - 占位符 对应shell |管道
    files: Vec<String>,

    /// Number lines
    #[arg(
        short('n'),
        long("number"),
        conflicts_with("number_nonblank_lines") // 互斥参数 *****
    )]
    number_lines: bool,

    /// Number non-blank lines
    #[arg(short('b'), long("number-nonblank"))]
    number_nonblank_lines: bool,
}


// pub fn get_args() -> MyResult<Config> {
//     // 解析命令行参数
//     // 这会自动处理 --help、--version 等标准选项
//     let args = Args::parse();
    
//     Ok(Config{
//         files:args.files,
//         number_lines:args.number_lines,
//         number_nonblank_lines:args.number_nonblank_lines,
//     })
// }


pub fn get_args() -> MyResult<Config> {
    // 解析命令行参数
    // 这会自动处理 --help、--version 等标准选项
    let matches = Command::new("catr")
                .version("0.1.0")
                .author("author <author@email.com>")
                .about("Rust cat")
                .arg(
                    Arg::new("files")
                    .value_name("FILE")
                    .help("Input file(s)")
                    .num_args(1..) // 至少一个参数
                    .default_value("-"),
                )
                .arg(
                    Arg::new("number_lines")
                    .short('n')
                    .long("number")
                    .help("Number lines")
                    .action(ArgAction::SetTrue) // 设置一个布尔值,出现即为true
                    .conflicts_with("number_nonblank_lines"),
                )
                .arg(
                    Arg::new("number_nonblank_lines")
                    .short('b')
                    .long("number-nonblank")
                    .help("Number non-blank lines")
                    .action(ArgAction::SetTrue) // 设置一个布尔值,出现即为true
                )
                .get_matches(); // 获取匹配
    
    // // 从matches中提取值
    // let files: Vec<String> = matches
    //     .get_many("files")
    //     .unwrap_or_default() // 如果不存在,返回默认值(注意:我们有default_value,所以应该总是有值)
    //     .cloned()
    //     .collect();

    // let number_lines = matches.get_flag("number_lines");
    // let number_nonblank_lines = matches.get_flag("number_nonblank_lines");

    // Ok(Config {
    //     files,
    //     number_lines,
    //     number_nonblank_lines,
    // })

    Ok(Config {
    files:matches
            .get_many("files")           // Option<ValuesRef<'_, String>>
            .expect("text is required") // ValuesRef<'_, String> (如果None则panic) 与unwrap类似,但如果结果是None,则会panic并显示指定的错误信息"text is required",有助于调试。
            .cloned()                   // 创建一个新的迭代器,其元素类型为String(通过克隆每个元素);
            // 迭代器的元素类型是&String(字符串的引用)
            // String实现了Clone trait
            // 克隆，从而将&String转换为String
            .collect(), // 将迭代器收集为Vec<String>
       

    number_lines:matches.get_flag("number_lines"),

    number_nonblank_lines:matches.get_flag("number_nonblank_lines"),
    })
}




