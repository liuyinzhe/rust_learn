use anyhow::Result;
use clap::{builder::PossibleValue, ArgAction, Parser, ValueEnum};
use regex::Regex;
use walkdir::{DirEntry, WalkDir}; // DirEntry 类型, WalkDir 遍历

// cargo add  clap --features derive
// cargo add anyhow
// cargo add regex
// cargo add walkdir

// cargo add --dev anyhow assert_cmd predicates pretty_assertions rand tempfile

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `find`
struct Args {
    /// Search path(s)
    #[arg(value_name = "PATH", default_value = ".")]
    paths: Vec<String>, // 可以接受多个搜索路径

    /// Names
    #[arg(
        short('n'),
        long("name"),
        value_name = "NAME",
        value_parser(Regex::new),    // 将字符串参数解析为正则表达式
        action(ArgAction::Append),   // 允许多次使用该选项
        num_args(0..)                // 接受0个或多个参数
    )]
    names: Vec<Regex>,               // 存储多个正则表达式

    /// Entry types
    #[arg(
        short('t'),
        long("type"),
        value_name = "TYPE",
        value_parser(clap::value_parser!(EntryType)), // 自定义类型解析, 使用枚举类型指定解析后的类型
        // clap::value_parser!() 宏 指定解析 枚举
        action(ArgAction::Append),
        num_args(0..)
    )]
    entry_types: Vec<EntryType>,      // Vec<EntryType> 向量
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum EntryType {
    Dir,    // 目录
    File,   // 文件
    Link,   // 符号链接
}
// ValueEnum trait 让 EntryType 可以被 clap 直接用作命令行参数类型
// 为 EntryType 枚举实现 ValueEnum trait
impl ValueEnum for EntryType {
    fn value_variants<'a>() -> &'a [Self] {
        &[EntryType::Dir, EntryType::File, EntryType::Link]
    }
    // PossibleValue 定义了在命令行中使用的实际值
    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            EntryType::Dir => PossibleValue::new("d"),
            EntryType::File => PossibleValue::new("f"),
            EntryType::Link => PossibleValue::new("l"),
        })
    }
}

// --------------------------------------------------
fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

// --------------------------------------------------
fn run(args: Args) -> Result<()> {
    // 文件类型判断 闭包函数
    let type_filter = |entry: &DirEntry| {
        args.entry_types.is_empty()
            || args.entry_types.iter().any(|entry_type| match entry_type { // 文件为空或者符合任何类型
                EntryType::Link => entry.file_type().is_symlink(), // DirEntry.file_type().is_symlink()
                EntryType::Dir => entry.file_type().is_dir(),
                EntryType::File => entry.file_type().is_file(),
            })
    };
    // 文件名称判断 闭包函数
    let name_filter = |entry: &DirEntry| {
        args.names.is_empty() // 参数为空
            || args
                .names
                .iter()       // 遍历所有正则表达式
                .any(|re| re.is_match(&entry.file_name().to_string_lossy())) //to_string_lossy()  // 转换为字符串(处理非法UTF-8)
    };

    // 遍历路径
    for path in &args.paths {
        let entries = WalkDir::new(path)
            .into_iter() // 转为迭代器
            .filter_map(|e| match e { // 过滤处理 
                Err(e) => {
                    eprintln!("{e}");
                    None
                }
                Ok(entry) => Some(entry),
            }) // Ok返回 entry
            .filter(type_filter) // 应用类型过滤
            .filter(name_filter) // 应用名称过滤
            .map(|entry| entry.path().display().to_string()) // 转换为路径字符串
            .collect::<Vec<_>>(); // 收集结果

        println!("{}", entries.join("\n"));
    }

    Ok(())
}
