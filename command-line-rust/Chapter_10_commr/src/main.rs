use crate::Column::*;
use anyhow::{anyhow, bail, Result}; // 宏anyhow!()转化错误 bail!()包装错误 //Result 类型
use clap::{ArgAction, Parser};
use std::{
    cmp::Ordering::*, // 导入 Ordering 枚举的所有变体 (Equal, Less, Greater)
    fs::File,
    io::{self, BufRead, BufReader},
};

// cargo add  clap --features derive
// cargo add anyhow

// cargo add --dev assert_cmd predicates pretty_assertions rand sys_info

// 显示只在文件1中的行、只在文件2中的行、两个文件共有的行
#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `comm`
struct Args {
    /// Input file 1
    #[arg()]
    file1: String,

    /// Input file 2
    #[arg()]
    file2: String,

    /// printing of column 1
    #[arg(short('1'), action(ArgAction::SetFalse))]
    show_col1: bool,    // 禁止输出只在文件1中的行

    /// Suppress printing of column 2
    #[arg(short('2'), action(ArgAction::SetFalse))]
    show_col2: bool,    // 禁止输出只在文件2中的行

    /// Suppress printing of column 3
    #[arg(short('3'), action(ArgAction::SetFalse))]
    show_col3: bool,    // 禁止输出第3列(两个文件共有的行)

    /// Case-insensitive comparison of lines
    #[arg(short)]
    insensitive: bool, // 不区分大小写

    /// Output delimiter
    #[arg(short, long("output-delimiter"), default_value = "\t")]
    delimiter: String, // 输出分隔符
}

enum Column<'a> { // 生命周期贯穿 枚举Column
    Col1(&'a str),
    Col2(&'a str),
    Col3(&'a str),
}

// --------------------------------------------------
fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

// --------------------------------------------------
fn run(args: Args) -> Result<()> {
    let file1 = &args.file1;
    let file2 = &args.file2;

    // 检查两个文件是否都是 STDIN
    if file1 == "-" && file2 == "-" {
        bail!(r#"Both input files cannot be STDIN ("-")"#);
    }

    // 创建一个闭包函数来处理大小写敏感性
    // 如果设置了不区分大小写，将行转换为小写，否则保持原样
    let case = |line: String| {
        if args.insensitive {
            line.to_lowercase() // 转换为小写
        } else {
            line                // 原样返回
        }
    };

    // 打开两个文件并创建行的迭代器
    // open(file1)?: 打开文件，如果出错则返回错误
    // .lines(): 获取行的迭代器, 不包含 \r\n
    // .map_while(Result::ok): 过滤掉读取错误，只保留成功读取的行
    // .map(case): 应用大小写处理闭包,对vec 批量处理
    let mut lines1 = open(file1)?.lines().map_while(Result::ok).map(case);
    let mut lines2 = open(file2)?.lines().map_while(Result::ok).map(case);

    let print = |col: Column| {
        let mut columns = vec![];
        match col { // 匹配对应的类型, 如果这个类型有参数不输出只有当前文件的行,则存储
            Col1(val) => {
                if args.show_col1 {  // 是否显示第1列 第一个文件单独有的行
                    columns.push(val);
                }
            }
            Col2(val) => {
                if args.show_col2 {
                    if args.show_col1 {
                        columns.push(""); // 是否显示第2列 比较则只push "" 空字符
                    }
                    columns.push(val);
                }
            }
            Col3(val) => {
                if args.show_col3 {  // 禁止比较则两个类型都 push "" 空字符
                    if args.show_col1 {
                        columns.push("");
                    }
                    if args.show_col2 {
                        columns.push("");
                    }
                    columns.push(val);
                }
            }
        };

        // 如果有内容要输出，使用指定的分隔符连接并打印
        if !columns.is_empty() {
            println!("{}", columns.join(&args.delimiter));
        }
    };
    
    // 获取两个文件的第一行
    let mut line1 = lines1.next();
    let mut line2 = lines2.next();

    while line1.is_some() || line2.is_some() {
        match (&line1, &line2) {
            (Some(val1), Some(val2)) => match val1.cmp(val2) {  // 两者比较
                Equal => {
                    // 行内容相等:属于第3列(两个文件共有)
                    print(Col3(val1));
                    // 两个文件都前进到下一行
                    line1 = lines1.next();
                    line2 = lines2.next();
                }
                Less => {
                    print(Col1(val1));
                    line1 = lines1.next();
                }
                Greater => {
                    print(Col2(val2));
                    line2 = lines2.next();
                }
            },
            (Some(val1), None) => {
                print(Col1(val1));
                line1 = lines1.next();
            }
            (None, Some(val2)) => {
                print(Col2(val2));
                line2 = lines2.next();
            }
            _ => (),
        }
    }

    Ok(())
}

// --------------------------------------------------
fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(
            File::open(filename).map_err(|e| anyhow!("{filename}: {e}"))?,
        ))),
    }
}
