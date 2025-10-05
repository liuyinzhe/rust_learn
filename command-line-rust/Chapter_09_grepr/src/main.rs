use anyhow::{anyhow, Result}; // 宏anyhow!()转化错误 //Result 类型
use clap::Parser;
use regex::{Regex, RegexBuilder};
use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader},
    mem,
};
use walkdir::WalkDir;

// cargo add  clap --features derive
// cargo add anyhow
// cargo add regex
// cargo add walkdir

// cargo add --dev assert_cmd predicates pretty_assertions rand sys_info

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `grep`
struct Args {
    /// Search pattern
    #[arg()]
    pattern: String,

    /// Input file(s)
    #[arg(default_value = "-", value_name = "FILE")]
    files: Vec<String>,

    /// Case-insensitive
    #[arg(short, long)]
    insensitive: bool,  // 忽略大小写

    /// Recursive search
    #[arg(short, long)]
    recursive: bool,    // 递归搜索目录

    /// Count occurrences
    #[arg(short, long)]
    count: bool,        // 统计匹配行数

    /// Invert match
    #[arg(short('v'), long("invert-match"))]
    invert: bool,       // 反转匹配(显示不匹配的行)
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
    // 检验正则表达式是否正规
    let pattern = RegexBuilder::new(&args.pattern)
        .case_insensitive(args.insensitive) // 是否忽略大小写
        .build() // build 检验正则表达式是否正规
        .map_err(|_| anyhow!(r#"Invalid pattern "{}""#, args.pattern))?;

    let entries = find_files(&args.files, args.recursive);
    let num_files = entries.len();  // 文件数量
    let print = |fname: &str, val: &str| {  // 闭包函数 文件前缀fname,文件名val
        if num_files > 1 {
            print!("{fname}:{val}");
        } else {
            print!("{val}");
        }
    };

    for entry in entries {
        match entry {
            Err(e) => eprintln!("{e}"), // 验证文件有效
            Ok(filename) => match open(&filename) { // 有效不报错就打开文件
                Err(e) => eprintln!("{filename}: {e}"),
                Ok(file) => match find_lines(file, &pattern, args.invert) {  // 成功就按行查找
                    Err(e) => eprintln!("{e}"),
                    Ok(matches) => { // 成功找到具体行, 返回值赋值给 matches
                        if args.count {
                            print(&filename, &format!("{}\n", matches.len()));
                        } else {
                            for line in &matches {
                                print(&filename, line);
                            }
                        }
                    }
                },
            },
        }
    }

    Ok(())
}

// --------------------------------------------------
fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

// --------------------------------------------------
fn find_lines<T: BufRead>(
    mut file: T,
    pattern: &Regex,
    invert: bool,
) -> Result<Vec<String>> {
    let mut matches = vec![];
    let mut line = String::new();

    loop {
        let bytes = file.read_line(&mut line)?; // 包含\r\n
        if bytes == 0 { // 读取到 EOF
            break;
        }
        // 使用异或操作实现反转匹配逻辑：
        // - 如果invert为false:pattern匹配成功时条件为true
        // - 如果invert为true:pattern匹配失败时条件为true
        if pattern.is_match(&line) ^ invert {
            matches.push(mem::take(&mut line));
            // mem::take取出line的内容(移动)，同时在原位置放入空字符串
            // 这样可以避免复制字符串，提高性能
        }
        line.clear();
    }

    Ok(matches)
}

// --------------------------------------------------
fn find_files(paths: &[String], recursive: bool) -> Vec<Result<String>> {
    let mut results = vec![];

    for path in paths {
        match path.as_str() {
            "-" => results.push(Ok(path.to_string())),
            _ => match fs::metadata(path) { // 获取文件元数据
                Ok(metadata) => { // 成功就将 fs::metadata(path) 赋值给 metadata
                    if metadata.is_dir() { // 如果是目录
                        if recursive { // 是否遍历
                            for entry in WalkDir::new(path)
                                .into_iter()
                                .flatten()  // 过滤掉错误条目
                                .filter(|e| e.file_type().is_file()) // 只保留文件
                            {
                                results.push(Ok(entry
                                    .path()
                                    .display()
                                    .to_string()));
                            }
                        } else {
                            results // 传入错误
                                .push(Err(anyhow!("{path} is a directory")));
                        }
                    } else if metadata.is_file() { // 如果是文件
                        results.push(Ok(path.to_string())); // 存入Ok  Result(Ok(),Err())
                    }// // 这里没有处理其他类型的文件(如符号链接)
                }
                Err(e) => results.push(Err(anyhow!("{path}: {e}"))), //  fs::metadata(path) 失败 存入报错
            },
        }
    }

    results
}

// --------------------------------------------------
// #[cfg(test)]
// mod tests {
//     use super::{find_files, find_lines};
//     use pretty_assertions::assert_eq;
//     use rand::{distributions::Alphanumeric, Rng};
//     use regex::{Regex, RegexBuilder};
//     use std::io::Cursor;

//     #[test]
//     fn test_find_lines() {
//         let text = b"Lorem\nIpsum\r\nDOLOR";

//         // The pattern _or_ should match the one line, "Lorem"
//         let re1 = Regex::new("or").unwrap();
//         let matches = find_lines(Cursor::new(&text), &re1, false);
//         assert!(matches.is_ok());
//         assert_eq!(matches.unwrap().len(), 1);

//         // When inverted, the function should match the other two lines
//         let matches = find_lines(Cursor::new(&text), &re1, true);
//         assert!(matches.is_ok());
//         assert_eq!(matches.unwrap().len(), 2);

//         // This regex will be case-insensitive
//         let re2 = RegexBuilder::new("or")
//             .case_insensitive(true)
//             .build()
//             .unwrap();

//         // The two lines "Lorem" and "DOLOR" should match
//         let matches = find_lines(Cursor::new(&text), &re2, false);
//         assert!(matches.is_ok());
//         assert_eq!(matches.unwrap().len(), 2);

//         // When inverted, the one remaining line should match
//         let matches = find_lines(Cursor::new(&text), &re2, true);
//         assert!(matches.is_ok());
//         assert_eq!(matches.unwrap().len(), 1);
//     }

//     #[test]
//     fn test_find_files() {
//         // Verify that the function finds a file known to exist
//         let files =
//             find_files(&["./tests/inputs/fox.txt".to_string()], false);
//         assert_eq!(files.len(), 1);
//         assert_eq!(files[0].as_ref().unwrap(), "./tests/inputs/fox.txt");

//         // The function should reject a directory without the recursive option
//         let files = find_files(&["./tests/inputs".to_string()], false);
//         assert_eq!(files.len(), 1);
//         if let Err(e) = &files[0] {
//             assert_eq!(e.to_string(), "./tests/inputs is a directory");
//         }

//         // Verify the function recurses to find four files in the directory
//         let res = find_files(&["./tests/inputs".to_string()], true);
//         let mut files: Vec<String> = res
//             .iter()
//             .map(|r| r.as_ref().unwrap().replace("\\", "/"))
//             .collect();
//         files.sort();
//         assert_eq!(files.len(), 4);
//         assert_eq!(
//             files,
//             vec![
//                 "./tests/inputs/bustle.txt",
//                 "./tests/inputs/empty.txt",
//                 "./tests/inputs/fox.txt",
//                 "./tests/inputs/nobody.txt",
//             ]
//         );

//         // Generate a random string to represent a nonexistent file
//         let bad: String = rand::thread_rng()
//             .sample_iter(&Alphanumeric)
//             .take(7)
//             .map(char::from)
//             .collect();

//         // Verify that the function returns the bad file as an error
//         let files = find_files(&[bad], false);
//         assert_eq!(files.len(), 1);
//         assert!(files[0].is_err());
//     }
// }
