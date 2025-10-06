use crate::TakeValue::*;
use anyhow::{anyhow, bail, Result}; // 宏anyhow!()转化错误 bail!()包装错误 //Result 类型
use clap::Parser;
use once_cell::sync::OnceCell;  // once_cell::OnceCell:用于一次性初始化的静态变量
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Seek, SeekFrom},  // Seek,处理光标位置, SeekFrom 移动光标
};

// cargo add  clap --features derive
// cargo add anyhow once_cell regex

// cargo add --dev assert_cmd predicates pretty_assertions rand sys_info


#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `tail`
struct Args {
    /// Input file(s)
    #[arg(required = true)]
    files: Vec<String>,

    /// Number of lines
    #[arg(value_name = "LINES", short('n'), long, default_value = "10")]
    lines: String, // 注意: 这里类型是 String 而不是数字，为了支持 +N 语法

    /// Number of bytes
    #[arg(value_name = "BYTES", short('c'), long, conflicts_with("lines"))]
    bytes: Option<String>,

    /// Suppress headers
    #[arg(short, long)]
    quiet: bool,
}

static NUM_RE: OnceCell<Regex> = OnceCell::new();

#[derive(Debug, PartialEq)]
enum TakeValue {
    PlusZero,     // 表示 +0 特殊情况
    TakeNum(i64), // 自定义的数字类型
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
    let lines = parse_num(args.lines)
        .map_err(|e| anyhow!("illegal line count -- {e}"))?;

    let bytes = args
        .bytes
        .map(parse_num) // 对 Option 中的值应用 parse_num
        .transpose()    // 将 Result<Option> 转换为 Option<Result>
        .map_err(|e| anyhow!("illegal byte count -- {e}"))?;

    let num_files = args.files.len();
    for (file_num, filename) in args.files.iter().enumerate() {
        match File::open(filename) {
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(file) => {
                // 多文件时显示文件名头(除非 quiet 模式)
                if !args.quiet && num_files > 1 {
                    println!(
                        "{}==> {filename} <==",
                        if file_num > 0 { "\n" } else { "" },
                    );
                }
                // 获取文件总行数和字节数
                let (total_lines, total_bytes) = count_lines_bytes(filename)?;
                let file = BufReader::new(file);

                // 根据参数选择输出方式 #bytes 是 Option<TakeValue> 类型, Option<Some,None> match Some 或者None
                if let Some(num_bytes) = &bytes {
                    print_bytes(file, num_bytes, total_bytes)?;
                } else {
                    print_lines(file, &lines, total_lines)?;
                }
            }
        }
    }

    Ok(())
}

// --------------------------------------------------
fn parse_num(val: String) -> Result<TakeValue> {
    // 延迟初始化正则表达式(只编译一次)
    let num_re =
        NUM_RE.get_or_init(|| Regex::new(r"^([+-])?(\d+)$").unwrap());

    match num_re.captures(&val) {
        Some(caps) => {
            // 获取符号(默认为 -)
            let sign = caps.get(1).map_or("-", |m| m.as_str()); // Option<Match>.map_or(默认值,Some(Value).as_str())
            let signed_num =
                format!("{sign}{}", caps.get(2).unwrap().as_str());

            if let Ok(num) = signed_num.parse() {
                if sign == "+" && num == 0 {
                    Ok(PlusZero)     // 返回值 Ok()+0)
                } else {
                    Ok(TakeNum(num)) // 返回值 Ok(TakeNum(num))
                }
            } else { 
                bail!(val)
            }
        }
        _ => bail!(val),
    }
}

// --------------------------------------------------
// We have to specify the type and assign to a variable here because
// &['+', '-'] has the type &[char; 2], and we want to coerce it to
// a slice, not a reference to an array.
//
// One day in the future we will be able to say
// val.starts_with(['+', '-'].as_slice())
// but array_methods are currently an unstable nightly feature.
//fn parse_num(val: String) -> Result<TakeValue> {
//    let signs: &[char] = &['+', '-'];
//    let res = val
//        .starts_with(signs)
//        .then(|| val.parse())
//        .unwrap_or_else(|| val.parse().map(i64::wrapping_neg));

//    match res {
//        Ok(num) => {
//            if num == 0 && val.starts_with('+') {
//                Ok(PlusZero)
//            } else {
//                Ok(TakeNum(num))
//            }
//        }
//        _ => bail!(val),
//    }
//}

// --------------------------------------------------
fn count_lines_bytes(filename: &str) -> Result<(i64, i64)> {
    let mut file = BufReader::new(File::open(filename)?);
    let mut num_lines = 0;
    let mut num_bytes = 0;
    let mut buf = Vec::new();
    loop {
        let bytes_read = file.read_until(b'\n', &mut buf)?; // read_until,按照 \n 换行符为分割,返回读取的字节数,字节存储到buf
        if bytes_read == 0 { // EOF
            break;
        }
        num_lines += 1;
        num_bytes += bytes_read as i64;
        buf.clear();
    }
    Ok((num_lines, num_bytes))
}

// --------------------------------------------------
fn print_bytes<T: Read + Seek>( // 实现了 Read + Seek trait 的泛型 T
    mut file: T,
    num_bytes: &TakeValue,
    total_bytes: i64,
) -> Result<()> {
    if let Some(start) = get_start_index(num_bytes, total_bytes) {
        file.seek(SeekFrom::Start(start))?; // 光标跳转
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?; // 读取到末尾,存储给buffer
        if !buffer.is_empty() {
            print!("{}", String::from_utf8_lossy(&buffer)); // 转string ,处理非UTF-8内容
        }
    }

    Ok(())
}

// --------------------------------------------------
fn print_lines(
    mut file: impl BufRead,
    num_lines: &TakeValue,
    total_lines: i64,
) -> Result<()> {
    if let Some(start) = get_start_index(num_lines, total_lines) {
        let mut line_num = 0;
        let mut buf = Vec::new();
        loop {
            let bytes_read = file.read_until(b'\n', &mut buf)?; // read_until,按照 \n 换行符为分割,返回读取的字节数,字节存储到buf
            if bytes_read == 0 {
                break;
            }
            if line_num >= start {
                print!("{}", String::from_utf8_lossy(&buf));
            }
            line_num += 1;
            buf.clear();
        }
    }

    Ok(())
}

// --------------------------------------------------
fn get_start_index(take_val: &TakeValue, total: i64) -> Option<u64> {
    match take_val {
        PlusZero => {
            if total > 0 {
                Some(0)
            } else {
                None
            }
        }
        TakeNum(num) => {
            // num 的类型是&i64 ,对应数字引用 &0 0数字的引用地址
            if num == &0 || total == 0 || num > &total {
                None
            } else {
                let start = if num < &0 { total + num } else { num - 1 };
                Some(if start < 0 { 0 } else { start as u64 })
            }
        }
    }
}

// --------------------------------------------------
// #[cfg(test)]
// mod tests {
//     use super::{
//         count_lines_bytes, get_start_index, parse_num, TakeValue::*,
//     };
//     use pretty_assertions::assert_eq;

//     #[test]
//     fn test_count_lines_bytes() {
//         let res = count_lines_bytes("tests/inputs/one.txt");
//         assert!(res.is_ok());
//         let (lines, bytes) = res.unwrap();
//         assert_eq!(lines, 1);
//         assert_eq!(bytes, 24);

//         let res = count_lines_bytes("tests/inputs/twelve.txt");
//         assert!(res.is_ok());
//         let (lines, bytes) = res.unwrap();
//         assert_eq!(lines, 12);
//         assert_eq!(bytes, 63);
//     }

//     #[test]
//     fn test_get_start_index() {
//         // +0 from an empty file (0 lines/bytes) returns None
//         assert_eq!(get_start_index(&PlusZero, 0), None);

//         // +0 from a nonempty file returns an index that
//         // is one less than the number of lines/bytes
//         assert_eq!(get_start_index(&PlusZero, 1), Some(0));

//         // Taking 0 lines/bytes returns None
//         assert_eq!(get_start_index(&TakeNum(0), 1), None);

//         // Taking any lines/bytes from an empty file returns None
//         assert_eq!(get_start_index(&TakeNum(1), 0), None);

//         // Taking more lines/bytes than is available returns None
//         assert_eq!(get_start_index(&TakeNum(2), 1), None);

//         // When starting line/byte is less than total lines/bytes,
//         // return one less than starting number
//         assert_eq!(get_start_index(&TakeNum(1), 10), Some(0));
//         assert_eq!(get_start_index(&TakeNum(2), 10), Some(1));
//         assert_eq!(get_start_index(&TakeNum(3), 10), Some(2));

//         // When starting line/byte is negative and less than total,
//         // return total - start
//         assert_eq!(get_start_index(&TakeNum(-1), 10), Some(9));
//         assert_eq!(get_start_index(&TakeNum(-2), 10), Some(8));
//         assert_eq!(get_start_index(&TakeNum(-3), 10), Some(7));

//         // When the starting line/byte is negative and more than the total,
//         // return 0 to print the whole file
//         assert_eq!(get_start_index(&TakeNum(-20), 10), Some(0));
//     }

//     #[test]
//     fn test_parse_num() {
//         // All integers should be interpreted as negative numbers
//         let res = parse_num("3".to_string());
//         assert!(res.is_ok());
//         assert_eq!(res.unwrap(), TakeNum(-3));

//         // A leading "+" should result in a positive number
//         let res = parse_num("+3".to_string());
//         assert!(res.is_ok());
//         assert_eq!(res.unwrap(), TakeNum(3));

//         // An explicit "-" value should result in a negative number
//         let res = parse_num("-3".to_string());
//         assert!(res.is_ok());
//         assert_eq!(res.unwrap(), TakeNum(-3));

//         // Zero is zero
//         let res = parse_num("0".to_string());
//         assert!(res.is_ok());
//         assert_eq!(res.unwrap(), TakeNum(0));

//         // Plus zero is special
//         let res = parse_num("+0".to_string());
//         assert!(res.is_ok());
//         assert_eq!(res.unwrap(), PlusZero);

//         // Test boundaries
//         let res = parse_num(i64::MAX.to_string());
//         assert!(res.is_ok());
//         assert_eq!(res.unwrap(), TakeNum(i64::MIN + 1));

//         let res = parse_num((i64::MIN + 1).to_string());
//         assert!(res.is_ok());
//         assert_eq!(res.unwrap(), TakeNum(i64::MIN + 1));

//         let res = parse_num(format!("+{}", i64::MAX));
//         assert!(res.is_ok());
//         assert_eq!(res.unwrap(), TakeNum(i64::MAX));

//         let res = parse_num(i64::MIN.to_string());
//         assert!(res.is_ok());
//         assert_eq!(res.unwrap(), TakeNum(i64::MIN));

//         // A floating-point value is invalid
//         let res = parse_num("3.14".to_string());
//         assert!(res.is_err());
//         assert_eq!(res.unwrap_err().to_string(), "3.14");

//         // Any non-integer string is invalid
//         let res = parse_num("foo".to_string());
//         assert!(res.is_err());
//         assert_eq!(res.unwrap_err().to_string(), "foo");
//     }
// }
