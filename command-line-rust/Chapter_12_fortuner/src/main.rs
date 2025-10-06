use anyhow::{anyhow, bail, Result}; // 宏anyhow!()转化错误 bail!()包装错误 //Result 类型
use clap::Parser;
use rand::prelude::IndexedRandom; // 切片随机选择功能
use rand::{rngs::StdRng, RngCore, SeedableRng}; // 随机数生成器

use regex::RegexBuilder;
use std::{
    ffi::OsStr,
    fs::{self, File},
    io::{BufRead, BufReader},
    path::PathBuf,
};
use walkdir::WalkDir;

// cargo add  clap --features derive
// cargo add anyhow walkdir regex rand

// cargo add --dev assert_cmd predicates pretty_assertions rand sys_info


#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `fortune`
struct Args {
    /// Input files or directories
    #[arg(required(true), value_name = "FILE")]
    sources: Vec<String>, // 输入文件或目录

    /// Pattern
    #[arg(short('m'), long)]
    pattern: Option<String>,

    /// Case-insensitive pattern matching
    #[arg(short, long)]
    insensitive: bool, // 大小写不敏感匹配

    /// Random seed
    #[arg(short, long, value_parser(clap::value_parser!(u64)))]
    seed: Option<u64>,  // 随机种子
}

#[derive(Debug)]
struct Fortune {
    source: String,  // 来源文件名
    text: String,    // 文本内容
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
    let pattern = args.pattern
        .map(|val: String| {
            RegexBuilder::new(val.as_str())
                .case_insensitive(args.insensitive)
                .build()
                .map_err(|_| anyhow!(r#"Invalid --pattern "{val}""#))
        })
        .transpose()?; // 将 Option<Result> 转换为 Result<Option>
    
    // 查找所有输入路径对应的文件
    let files = find_files(&args.sources)?;
    // 从文件中读取所有内容
    let fortunes = read_fortunes(&files)?;

    match pattern {
        Some(pattern) => {
            let mut prev_source = None;
            for fortune in fortunes
                .iter()
                .filter(|fortune| pattern.is_match(&fortune.text))
            {
                if prev_source.as_ref().map_or(true, |s| s != &fortune.source)
                {
                    eprintln!("({})\n%", fortune.source);
                    prev_source = Some(fortune.source.clone());
                }
                println!("{}\n%", fortune.text);
            }
        }
        _ => { // 如果没有模式参数，随机选择一个
            println!(
                "{}",
                pick_fortune(&fortunes, args.seed)
                    .or_else(|| Some("No fortunes found".to_string()))
                    .unwrap()
            );
        }
    }

    Ok(())
}

// --------------------------------------------------
fn find_files(paths: &[String]) -> Result<Vec<PathBuf>> {
    let dat = OsStr::new("dat"); // 定义dat扩展名
    let mut files = vec![];

    for path in paths {
        match fs::metadata(path) {
            Err(e) => bail!("{path}: {e}"),
            Ok(_) => files.extend(
                // 使用walkdir递归遍历目录
                WalkDir::new(path)
                    .into_iter()
                    .filter_map(Result::ok)
                    .filter(|e| {
                        e.file_type().is_file()
                            && e.path().extension() != Some(dat)
                    })
                    .map(|e| e.path().into()), // into() 转换/包装为结果类型 Result<Vec<PathBuf>>
            ),
        }
    }

    files.sort();
    files.dedup();
    Ok(files)
}

// --------------------------------------------------
fn read_fortunes(paths: &[PathBuf]) -> Result<Vec<Fortune>> {
    let mut fortunes = vec![]; // 最终返回值
    let mut buffer = vec![]; // 临时缓冲区，用于累积多行文本

    for path in paths {
        // 获取basename
        let basename =
            path.file_name().unwrap().to_string_lossy().into_owned(); // into_owned(): 通过 clone()等方式获得所有权
        // 获取文件句柄
        let file = File::open(path)
            .map_err(|e| anyhow!("{}: {e}", path.to_string_lossy()))?;
        
        // 逐行读取文件
        for line in BufReader::new(file).lines().map_while(Result::ok) {
            if line == "%" {  // 遇到 % 分隔符,表示一个结束符
                if !buffer.is_empty() { // buffer 非空
                    fortunes.push(Fortune {
                        source: basename.clone(),
                        text: buffer.join("\n"),  // 多行buffer Vec内容 使用"\n" 连接字符串
                    });
                    buffer.clear();
                }
            } else {
                buffer.push(line.to_string()); // buffer 追加
            }
        }
    }

    Ok(fortunes)
}

// --------------------------------------------------
fn pick_fortune(fortunes: &[Fortune], seed: Option<u64>) -> Option<String> {
    let mut rng: Box<dyn RngCore> = match seed {
        Some(val) => Box::new(StdRng::seed_from_u64(val)), // val 确定随机种子
        _ => Box::new(rand::thread_rng()),  // 系统随机种子
    };
    // use rand::prelude::IndexedRandom;
    // 切片列表中随机选择一个 // 转为字符串 返回
    fortunes.choose(&mut rng).map(|f| f.text.to_string())
}

// --------------------------------------------------
// #[cfg(test)]
// mod tests {
//     use super::{find_files, pick_fortune, read_fortunes, Fortune};
//     use std::path::PathBuf;

//     #[test]
//     fn test_find_files() {
//         // Verify that the function finds a file known to exist
//         let res = find_files(&["./tests/inputs/jokes".to_string()]);
//         assert!(res.is_ok());

//         let files = res.unwrap();
//         assert_eq!(files.len(), 1);
//         assert_eq!(
//             files.get(0).unwrap().to_string_lossy(),
//             "./tests/inputs/jokes"
//         );

//         // Fails to find a bad file
//         let res = find_files(&["/path/does/not/exist".to_string()]);
//         assert!(res.is_err());

//         // Finds all the input files, excludes ".dat"
//         let res = find_files(&["./tests/inputs".to_string()]);
//         assert!(res.is_ok());

//         // Check number and order of files
//         let files = res.unwrap();
//         assert_eq!(files.len(), 5);
//         let first = files.get(0).unwrap().display().to_string();
//         assert!(first.contains("ascii-art"));
//         let last = files.last().unwrap().display().to_string();
//         assert!(last.contains("quotes"));

//         // Test for multiple sources, path must be unique and sorted
//         let res = find_files(&[
//             "./tests/inputs/jokes".to_string(),
//             "./tests/inputs/ascii-art".to_string(),
//             "./tests/inputs/jokes".to_string(),
//         ]);
//         assert!(res.is_ok());
//         let files = res.unwrap();
//         assert_eq!(files.len(), 2);
//         if let Some(filename) = files.first().unwrap().file_name() {
//             assert_eq!(filename.to_string_lossy(), "ascii-art".to_string())
//         }
//         if let Some(filename) = files.last().unwrap().file_name() {
//             assert_eq!(filename.to_string_lossy(), "jokes".to_string())
//         }
//     }

//     #[test]
//     fn test_read_fortunes() {
//         // Parses all the fortunes without a filter
//         let res = read_fortunes(&[PathBuf::from("./tests/inputs/jokes")]);
//         assert!(res.is_ok());

//         if let Ok(fortunes) = res {
//             // Correct number and sorting
//             assert_eq!(fortunes.len(), 6);
//             assert_eq!(
//                 fortunes.first().unwrap().text,
//                 "Q. What do you call a head of lettuce in a shirt and tie?\n\
//                 A. Collared greens."
//             );
//             assert_eq!(
//                 fortunes.last().unwrap().text,
//                 "Q: What do you call a deer wearing an eye patch?\n\
//                 A: A bad idea (bad-eye deer)."
//             );
//         }

//         // Filters for matching text
//         let res = read_fortunes(&[
//             PathBuf::from("./tests/inputs/jokes"),
//             PathBuf::from("./tests/inputs/quotes"),
//         ]);
//         assert!(res.is_ok());
//         assert_eq!(res.unwrap().len(), 11);
//     }

//     #[test]
//     fn test_pick_fortune() {
//         // Create a slice of fortunes
//         let fortunes = &[
//             Fortune {
//                 source: "fortunes".to_string(),
//                 text: "You cannot achieve the impossible without \
//                       attempting the absurd."
//                     .to_string(),
//             },
//             Fortune {
//                 source: "fortunes".to_string(),
//                 text: "Assumption is the mother of all screw-ups."
//                     .to_string(),
//             },
//             Fortune {
//                 source: "fortunes".to_string(),
//                 text: "Neckties strangle clear thinking.".to_string(),
//             },
//         ];

//         // Pick a fortune with a seed
//         assert_eq!(
//             pick_fortune(fortunes, Some(1)).unwrap(),
//             "Neckties strangle clear thinking.".to_string()
//         );
//     }
// }
