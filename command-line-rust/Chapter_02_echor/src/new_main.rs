// use clap::Parser;

// #[derive(Debug, Parser)]
// #[command(author, version, about)]
// /// Rust version of `echo`
// struct Args {
//     /// Input text
//     #[arg(required(true))]
//     text: Vec<String>,

//     /// Do not print newline
//     #[arg(short('n'))]
//     omit_newline: bool,
// }

// fn main() {
//     let args = Args::parse();
//     print!(
//         "{}{}",
//         args.text.join(" "),
//         if args.omit_newline { "" } else { "\n" }
//     );
// }

// cargo add  clap --features derive

use clap::Parser;

/// Rust version of `echo`
// 使用 derive 宏自动实现 Parser trait,这样结构体就能解析命令行参数
// Debug trait 用于打印调试信息
#[derive(Debug, Parser)]
// 为命令行应用添加元数据：作者、版本和描述
// 这些信息会自动从 Cargo.toml 中获取
#[command(author, version, about)]
struct Args {
    /// Input text
    // 这个字段是必需的，必须提供至少一个参数
    // required = true 表示这个参数是必须的
    // num_args 指定这个参数可以接受多个值(1个或多个)
    #[arg(required = true, num_args = 1..)]
    text: Vec<String>,

    /// Do not print newline
    // short = 'n' 表示可以用 -n 短标志来指定这个选项
    // long = "no-newline" 表示也可以用 --no-newline 长标志（可选，这里没写）
    // action = clap::ArgAction::SetTrue 表示当出现这个标志时，将其设置为 true
    #[arg(short = 'n', action = clap::ArgAction::SetTrue)]
    omit_newline: bool,
}

fn main() {
    // 解析命令行参数
    // 这会自动处理 --help、--version 等标准选项
    let args = Args::parse();
    
    // 打印结果
    // 将 text 向量中的字符串用空格连接起来
    // 根据 omit_newline 决定是否换行
    print!(
        "{}{}",
        args.text.join(" "),
        if args.omit_newline { "" } else { "\n" }
    );
}