// cargo add  clap --features derive
// cargo add --dev assert_cmd predicates
// cargo add --dev rand
// cargo add anyhow

use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `wc`
struct Args {
    /// Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,  // 文件列表，默认为 "-" 表示标准输入

    /// Show line count
    #[arg(short, long)] // 短选项 -l, 长选项 --lines
    lines: bool,

    /// Show word count
    #[arg(short, long)]
    words: bool,

    /// Show byte count
    #[arg(short('c'), long)] // short 指定 短选项c
    bytes: bool,

    /// Show character count
    #[arg(short('m'), long, conflicts_with("bytes"))]
    chars: bool,
}

#[derive(Debug, PartialEq)]
struct FileInfo {
    num_lines: usize,  // 行数
    num_words: usize,  // 单词数
    num_bytes: usize,  // 字节数
    num_chars: usize,  // 字符数
}

// --------------------------------------------------
fn main() {
    if let Err(e) = run(Args::parse()) {   // 解析参数并运行
        eprintln!("{e}");
        std::process::exit(1);             // 非零退出码表示错误
    }
}

// --------------------------------------------------
fn run(mut args: Args) -> Result<()> {
    // 如果没有指定任何统计选项，默认显示行数、单词数、字节数,都是true
    if [args.words, args.bytes, args.chars, args.lines]
        .iter()
        .all(|v| v == &false) //all 检查是否所有元素都等于false // |v| v == &false 是一个闭包,比较每个元素引用与false的引用
    {
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }
    // 初始化总计变量
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;

    // 遍历文件
    for filename in &args.files {
        match open(filename) {
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(file) => {
                if let Ok(info) = count(file) {  // 返回 结构体FileInfo 获取信息 num_lines, num_words, num_bytes, num_chars
                    println!(
                        "{}{}{}{}{}",
                        format_field(info.num_lines, args.lines), // 格式化 STDOUT 输出字符串
                        format_field(info.num_words, args.words),
                        format_field(info.num_bytes, args.bytes),
                        format_field(info.num_chars, args.chars),
                        if filename == "-" {
                            "".to_string()
                        } else {
                            format!(" {filename}")
                        },
                    );
                    // 累加每行的计数
                    total_lines += info.num_lines;
                    total_words += info.num_words;
                    total_bytes += info.num_bytes;
                    total_chars += info.num_chars;
                }
            }
        }
    }
    // 如果处理了多个文件，显示总计
    if args.files.len() > 1 {
        println!(
            "{}{}{}{} total",
            format_field(total_lines, args.lines),
            format_field(total_words, args.words),
            format_field(total_bytes, args.bytes),
            format_field(total_chars, args.chars)
        );
    }

    Ok(())
}

// --------------------------------------------------
fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),         // 从STDIN 读取
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))), // 从普通文件读取
    }
}

// --------------------------------------------------
fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{value:>8}") // 右对齐,宽度为8
    } else {
        "".to_string()  // 返回空字符
    }
}

// --------------------------------------------------
fn count(mut file: impl BufRead) -> Result<FileInfo> { // 统计文件中的信息
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut line = String::new();

    loop {
        let line_bytes = file.read_line(&mut line)?; // 每次读取一行,包含 \r\n; line_bytes 获得长度, 缓存入line
        if line_bytes == 0 {    // 当读到 EOF
            break;
        }
        num_bytes += line_bytes;
        num_lines += 1;
        num_words += line.split_whitespace().count(); // split_whitespace 按照空白符拆分计数单词
        num_chars += line.chars().count();  // Iterator trait 的 Chars 迭代器
        line.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

// --------------------------------------------------
#[cfg(test)]
mod tests {
    use super::{count, format_field, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world.\nI just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 2,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }

    #[test]
    fn test_format_field() {
        assert_eq!(format_field(1, false), "");
        assert_eq!(format_field(3, true), "       3");
        assert_eq!(format_field(10, true), "      10");
    }
}
