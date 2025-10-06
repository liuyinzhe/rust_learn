mod owner;  // 声明 owner 模块(对应 owner.rs 文件)

use anyhow::Result;
use chrono::{DateTime, Local};  // 日期时间处理
use clap::Parser;
use owner::Owner;   // 自定义的文件权限所有者枚举
use std::{fs, os::unix::fs::MetadataExt, path::PathBuf}; // 文件系统和路径操作
use tabular::{Row, Table};  // 表格格式化输出
use users::{get_group_by_gid, get_user_by_uid};     // 用户和组信息查询

// cargo add  clap --features derive
// cargo add anyhow chrono regex tabular users

// cargo add --dev assert_cmd predicates pretty_assertions rand sys_info

#[derive(Debug, Parser)]  // 自动实现 Parser trait,用于命令行解析 into()
#[command(author, version, about)]
/// Rust version of `ls`
struct Args {
    /// Files and/or directories
    #[arg(default_value = ".")]
    paths: Vec<String>,

    /// Long listing
    #[arg(short, long)]
    long: bool,

    /// Show all files
    #[arg(short('a'), long("all"))]
    show_hidden: bool,
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
    // 根据参数查找文件
    let paths = find_files(&args.paths, args.show_hidden)?;
    // 长格式输出
    if args.long {
        println!("{}", format_output(&paths)?);
    } else { // 简单输出，只显示路径
        for path in paths {
            println!("{}", path.display());
        }
    }
    Ok(())
}

// --------------------------------------------------
fn find_files(paths: &[String], show_hidden: bool) -> Result<Vec<PathBuf>> {
    let mut results = vec![];
    for name in paths {
        match fs::metadata(name) { // 获取文件元数据
            Err(e) => eprintln!("{name}: {e}"),
            Ok(meta) => {
                if meta.is_dir() { // 如果是目录
                    for entry in fs::read_dir(name)? { // read_dir 读取目录内容
                        let entry = entry?;
                        let path = entry.path();
                        // 检查是否为隐藏文件（以 . 开头）
                        let is_hidden =
                            path.file_name().map_or(false, |file_name| {
                                file_name.to_string_lossy().starts_with('.')
                            });
                        // 如果不是隐藏文件，或者设置了显示隐藏文件，则添加到结果中
                        if !is_hidden || show_hidden {
                            results.push(entry.path());
                        }
                    }
                } else { // 普通文件
                    results.push(PathBuf::from(name));
                }
            }
        }
    }

    Ok(results)
}

// --------------------------------------------------
fn format_output(paths: &[PathBuf]) -> Result<String> {
    // 定义表格格式：{:<} 左对齐，{:>} 右对齐
    // 对应：文件类型、权限、链接数、用户、组、大小、修改时间、文件名
    //         1   2     3     4     5     6     7     8
    let fmt = "{:<}{:<}  {:>}  {:<}  {:<}  {:>}  {:<}  {:<}";
    let mut table = Table::new(fmt);

    for path in paths {
        let metadata = path.metadata()?; // 获取文件元数据

        // 获取用户信息
        let uid = metadata.uid(); // 用户ID
        let user = get_user_by_uid(uid) // 获取用户名
            .map(|u| u.name().to_string_lossy().into_owned())
            .unwrap_or_else(|| uid.to_string()); // 如果找不到用户，显示UID

        // 获取组信息
        let gid = metadata.gid(); // 组ID
        let group = get_group_by_gid(gid)
            .map(|g| g.name().to_string_lossy().into_owned())  // 获取组名 // into_owned 通过clone 获得所有权
            .unwrap_or_else(|| gid.to_string()); // 如果找不到组，显示GID
        
        // 确定文件类型：目录或普通文件
        let file_type = if path.is_dir() { "d" } else { "-" };
        // 格式化权限字符串
        let perms = format_mode(metadata.mode());
        // 转换修改时间
        let modified: DateTime<Local> = DateTime::from(metadata.modified()?);

        // 添加行到表格
        table.add_row(
            Row::new()
                .with_cell(file_type) // 1. 文件类型
                .with_cell(perms) // 2. 权限
                .with_cell(metadata.nlink()) // 3. 硬链接数
                .with_cell(user) // 4. 用户名
                .with_cell(group) // 5. 组名
                .with_cell(metadata.len()) // 6. 文件大小(字节)
                .with_cell(modified.format("%b %d %y %H:%M")) // 7. 修改时间
                .with_cell(path.display()), // 8. 文件名
        );
    }

    Ok(format!("{table}"))
}

// --------------------------------------------------
/// Given a file mode in octal format like 0o751,
/// return a string like "rwxr-x--x"
// 将八进制的文件模式(如 0o751)转换为字符串(如 "rwxr-x--x")
fn format_mode(mode: u32) -> String {
    format!(
        "{}{}{}",
        mk_triple(mode, Owner::User),
        mk_triple(mode, Owner::Group),
        mk_triple(mode, Owner::Other),
    )
}

// --------------------------------------------------
/// Given an octal number like 0o500 and an [`Owner`],
/// return a string like "r-x"
// 根据所有者和文件模式生成权限三元组(如 "r-x")
fn mk_triple(mode: u32, owner: Owner) -> String {
    let [read, write, execute] = owner.masks();
    format!(
        "{}{}{}",
        if mode & read == 0 { "-" } else { "r" },
        if mode & write == 0 { "-" } else { "w" },
        if mode & execute == 0 { "-" } else { "x" },
    )
}

// --------------------------------------------------
// #[cfg(test)]
// mod test {
//     use super::{find_files, format_mode, format_output, mk_triple, Owner};
//     use pretty_assertions::assert_eq;
//     use std::path::PathBuf;

//     #[test]
//     fn test_find_files() {
//         // Find all non-hidden entries in a directory
//         let res = find_files(&["tests/inputs".to_string()], false);
//         assert!(res.is_ok());
//         let mut filenames: Vec<_> = res
//             .unwrap()
//             .iter()
//             .map(|entry| entry.display().to_string())
//             .collect();
//         filenames.sort();
//         assert_eq!(
//             filenames,
//             [
//                 "tests/inputs/bustle.txt",
//                 "tests/inputs/dir",
//                 "tests/inputs/empty.txt",
//                 "tests/inputs/fox.txt",
//             ]
//         );

//         // Any existing file should be found even if hidden
//         let res = find_files(&["tests/inputs/.hidden".to_string()], false);
//         assert!(res.is_ok());
//         let filenames: Vec<_> = res
//             .unwrap()
//             .iter()
//             .map(|entry| entry.display().to_string())
//             .collect();
//         assert_eq!(filenames, ["tests/inputs/.hidden"]);

//         // Test multiple path arguments
//         let res = find_files(
//             &[
//                 "tests/inputs/bustle.txt".to_string(),
//                 "tests/inputs/dir".to_string(),
//             ],
//             false,
//         );
//         assert!(res.is_ok());
//         let mut filenames: Vec<_> = res
//             .unwrap()
//             .iter()
//             .map(|entry| entry.display().to_string())
//             .collect();
//         filenames.sort();
//         assert_eq!(
//             filenames,
//             ["tests/inputs/bustle.txt", "tests/inputs/dir/spiders.txt"]
//         );
//     }

//     #[test]
//     fn test_find_files_hidden() {
//         // Find all entries in a directory including hidden
//         let res = find_files(&["tests/inputs".to_string()], true);
//         assert!(res.is_ok());
//         let mut filenames: Vec<_> = res
//             .unwrap()
//             .iter()
//             .map(|entry| entry.display().to_string())
//             .collect();
//         filenames.sort();
//         assert_eq!(
//             filenames,
//             [
//                 "tests/inputs/.hidden",
//                 "tests/inputs/bustle.txt",
//                 "tests/inputs/dir",
//                 "tests/inputs/empty.txt",
//                 "tests/inputs/fox.txt",
//             ]
//         );
//     }

//     fn long_match(
//         line: &str,
//         expected_name: &str,
//         expected_perms: &str,
//         expected_size: Option<&str>,
//     ) {
//         let parts: Vec<_> = line.split_whitespace().collect();
//         assert!(!parts.is_empty() && parts.len() <= 10);

//         let perms = parts.first().unwrap();
//         assert_eq!(perms, &expected_perms);

//         if let Some(size) = expected_size {
//             let file_size = parts.get(4).unwrap();
//             assert_eq!(file_size, &size);
//         }

//         let display_name = parts.last().unwrap();
//         assert_eq!(display_name, &expected_name);
//     }

//     #[test]
//     fn test_format_output_one() {
//         let bustle_path = "tests/inputs/bustle.txt";
//         let bustle = PathBuf::from(bustle_path);

//         let res = format_output(&[bustle]);
//         assert!(res.is_ok());

//         let out = res.unwrap();
//         let lines: Vec<&str> =
//             out.split('\n').filter(|s| !s.is_empty()).collect();
//         assert_eq!(lines.len(), 1);

//         let line1 = lines.first().unwrap();
//         long_match(line1, bustle_path, "-rw-r--r--", Some("193"));
//     }

//     #[test]
//     fn test_format_output_two() {
//         let res = format_output(&[
//             PathBuf::from("tests/inputs/dir"),
//             PathBuf::from("tests/inputs/empty.txt"),
//         ]);
//         assert!(res.is_ok());

//         let out = res.unwrap();
//         let mut lines: Vec<&str> =
//             out.split('\n').filter(|s| !s.is_empty()).collect();
//         lines.sort();
//         assert_eq!(lines.len(), 2);

//         let empty_line = lines.remove(0);
//         long_match(
//             empty_line,
//             "tests/inputs/empty.txt",
//             "-rw-r--r--",
//             Some("0"),
//         );

//         let dir_line = lines.remove(0);
//         long_match(dir_line, "tests/inputs/dir", "drwxr-xr-x", None);
//     }

//     #[test]
//     fn test_mk_triple() {
//         assert_eq!(mk_triple(0o751, Owner::User), "rwx");
//         assert_eq!(mk_triple(0o751, Owner::Group), "r-x");
//         assert_eq!(mk_triple(0o751, Owner::Other), "--x");
//         assert_eq!(mk_triple(0o600, Owner::Other), "---");
//     }

//     #[test]
//     fn test_format_mode() {
//         assert_eq!(format_mode(0o755), "rwxr-xr-x");
//         assert_eq!(format_mode(0o421), "r---w---x");
//     }
// }
