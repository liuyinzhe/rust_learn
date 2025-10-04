use anyhow::{anyhow, bail, Result}; // 宏anyhow!()转化错误 bail!()包装错误 //Result 类型
use clap::Parser;
use csv::{ReaderBuilder, StringRecord, WriterBuilder};
use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    num::NonZeroUsize,
    ops::Range,
};

// cargo add  clap --features derive
// cargo add anyhow
// cargo add regex
// cargo add csv

// cargo add --dev assert_cmd predicates pretty_assertions rand


#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `cut`
struct Args {
    /// Input file(s)
    #[arg(default_value = "-")]
    files: Vec<String>,

    /// Field delimiter
    #[arg(short, long, value_name = "DELIMITER", default_value = "\t")]
    delimiter: String,

    // flatten(): 将多维容器的迭代器或迭代器的迭代器，展开为线性的迭代器
    // 解构 为线性的迭代器
    // 多层嵌套的迭代器结构(如 Vec<Vec<i32>>)转换为单层 Vec<i32> 实现数据扁平化处理
    #[command(flatten)] // 使用flatten将ArgsExtract结构体的字段合并到Args中
    extract: ArgsExtract,
}

#[derive(Debug, clap::Args)]
#[group(required = true, multiple = false)] // 必须选择一种(required=true),三个选项是互斥(multiple=false)
struct ArgsExtract {
    /// Selected fields
    #[arg(short, long, value_name = "FIELDS")]
    fields: Option<String>,

    /// Selected bytes
    #[arg(short, long, value_name = "BYTES")]
    bytes: Option<String>,

    /// Selected chars
    #[arg(short, long, value_name = "CHARS")]
    chars: Option<String>,
}

// 类型别名
type PositionList = Vec<Range<usize>>;

#[derive(Debug)]
enum Extract {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
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
    // 检查分隔符是否为单字节
    let delim_bytes = args.delimiter.as_bytes();
    if delim_bytes.len() != 1 { // 长度检查
        bail!(r#"--delim "{}" must be a single byte"#, args.delimiter);
    }
    let delimiter: u8 = *delim_bytes.first().unwrap();

    let extract = if let Some(fields) =
        // extract 参数结果为 结构体ArgsExtract,去其中的 fields  Vec<Range<usize>> map 遍历
        args.extract.fields.map(parse_pos).transpose()? // Option 与 Result 转换包装
        // 将 Result<Option<T>, E> 转换为 Option<Result<T, E>>
    {
        Extract::Fields(fields)
    } else if let Some(bytes) =
        args.extract.bytes.map(parse_pos).transpose()?
    {
        Extract::Bytes(bytes)
    } else if let Some(chars) =
        args.extract.chars.map(parse_pos).transpose()?
    {
        Extract::Chars(chars)
    } else { // match 最终分支报错引发 panic
        unreachable!("Must have --fields, --bytes, or --chars");
    };

    // 处理文件
    for filename in &args.files {
        match open(filename) {
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(file) => match &extract {
                // 域
                Extract::Fields(field_pos) => {
                    // CSV 读取器配置
                    let mut reader = ReaderBuilder::new()
                        .delimiter(delimiter) // 设置分割符
                        .has_headers(false)   // 指定 CSV 文件没有标题行
                        .from_reader(file);   // 从打开的文件创建读取器

                    // CSV 写入器配置
                    let mut wtr = WriterBuilder::new()
                        .delimiter(delimiter)       // 设置分割符
                        .from_writer(io::stdout()); // 输出到标准输出

                    for record in reader.records() {
                        wtr.write_record(extract_fields( // extract_fields 提取指定列
                            &record?, field_pos,
                        ))?;
                    }
                }
                // 字节
                Extract::Bytes(byte_pos) => {  // Fields(PositionList) // type PositionList = Vec<Range<usize>>;
                    for line in file.lines() {
                        println!("{}", extract_bytes(&line?, byte_pos));
                    }
                }
                // 字符
                Extract::Chars(char_pos) => {
                    for line in file.lines() {
                        println!("{}", extract_chars(&line?, char_pos));
                    }
                }
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
// Parse an index from a string representation of an integer.
// Ensures the number is non-zero.
// Ensures the number does not start with '+'.
// Returns an index, which is a non-negative integer that is
// one less than the number represented by the original input.
fn parse_index(input: &str) -> Result<usize> {
    let value_error = || anyhow!(r#"illegal list value: "{input}""#); // 闭包函数
    input
        .starts_with('+')
        .then(|| Err(value_error())) // '+' 开头就报错
        .unwrap_or_else(|| {         // 否则
            input
                .parse::<NonZeroUsize>() // clap解析 为非0数字
                .map(|n| usize::from(n) - 1) // 数字减一
                .map_err(|_| value_error()) // 捕获错误,调用 闭包函数
        })
}

// --------------------------------------------------
fn parse_pos(range: String) -> Result<PositionList> {  // 1,3-5,7
    let range_re = Regex::new(r"^(\d+)-(\d+)$").unwrap();
    range
        .split(',') // 分割为多份
        .map(|val| {  // vec ['1','3-5']
            parse_index(val).map(|n| n..n + 1).or_else(|e| {  
                // 如果 parse_index 解析成功,则是一个未知的左开右闭区间 [n..n + 1] ,[0..1]
                // 否则 or_else
                range_re.captures(val).ok_or(e).and_then(|captures| { // 正则捕获两个数字 3-5
                    let n1 = parse_index(&captures[1])?;
                    let n2 = parse_index(&captures[2])?;
                    if n1 >= n2 {
                        bail!(
                            "First number in range ({}) \
                            must be lower than second number ({})",
                            n1 + 1,
                            n2 + 1
                        );
                    }
                    Ok(n1..n2 + 1)
                })
            })
        })
        .collect::<Result<_, _>>() // 收集数字 VEC
        .map_err(From::from) // 返回错误类型可能有多种 // 标准错误转换为自定义错误类型 
        // From::from就会自动将其转换为anyhow::Error
}

// --------------------------------------------------
fn extract_fields<'a>(
    record: &'a StringRecord,
    field_pos: &[Range<usize>],
) -> Vec<&'a str> {
    field_pos
        .iter()
        .cloned()
        // flat_map(): 将多维容器的迭代器或迭代器的迭代器,展开为线性的迭代器后,
        // 再进行map闭包运算,返回新的迭代器。
        .flat_map(|range| range.filter_map(|i| record.get(i))) // get 提取所需的
        .collect()
}

// --------------------------------------------------
fn extract_bytes(line: &str, byte_pos: &[Range<usize>]) -> String {
    let bytes = line.as_bytes();
    let selected: Vec<_> = byte_pos
        .iter()
        .cloned()
        // flat_map 展平所有范围 // filter_map 根据index从bytes获取元素
        .flat_map(|range| range.filter_map(|i| bytes.get(i)).copied())
        .collect();
    String::from_utf8_lossy(&selected).into_owned()
}

// --------------------------------------------------
fn extract_chars(line: &str, char_pos: &[Range<usize>]) -> String {
    let chars: Vec<_> = line.chars().collect();
    char_pos
        .iter()
        .cloned()
        .flat_map(|range| range.filter_map(|i| chars.get(i)))
        .collect()
}

// --------------------------------------------------
// #[cfg(test)]
// mod unit_tests {
//     use super::{extract_bytes, extract_chars, extract_fields, parse_pos};
//     use csv::StringRecord;
//     use pretty_assertions::assert_eq;

//     #[test]
//     fn test_parse_pos() {
//         // The empty string is an error
//         assert!(parse_pos("".to_string()).is_err());

//         // Zero is an error
//         let res = parse_pos("0".to_string());
//         assert!(res.is_err());
//         assert_eq!(
//             res.unwrap_err().to_string(),
//             r#"illegal list value: "0""#
//         );

//         let res = parse_pos("0-1".to_string());
//         assert!(res.is_err());
//         assert_eq!(
//             res.unwrap_err().to_string(),
//             r#"illegal list value: "0""#
//         );

//         // A leading "+" is an error
//         let res = parse_pos("+1".to_string());
//         assert!(res.is_err());
//         assert_eq!(
//             res.unwrap_err().to_string(),
//             r#"illegal list value: "+1""#,
//         );

//         let res = parse_pos("+1-2".to_string());
//         assert!(res.is_err());
//         assert_eq!(
//             res.unwrap_err().to_string(),
//             r#"illegal list value: "+1-2""#,
//         );

//         let res = parse_pos("1-+2".to_string());
//         assert!(res.is_err());
//         assert_eq!(
//             res.unwrap_err().to_string(),
//             r#"illegal list value: "1-+2""#,
//         );

//         // Any non-number is an error
//         let res = parse_pos("a".to_string());
//         assert!(res.is_err());
//         assert_eq!(
//             res.unwrap_err().to_string(),
//             r#"illegal list value: "a""#
//         );

//         let res = parse_pos("1,a".to_string());
//         assert!(res.is_err());
//         assert_eq!(
//             res.unwrap_err().to_string(),
//             r#"illegal list value: "a""#
//         );

//         let res = parse_pos("1-a".to_string());
//         assert!(res.is_err());
//         assert_eq!(
//             res.unwrap_err().to_string(),
//             r#"illegal list value: "1-a""#,
//         );

//         let res = parse_pos("a-1".to_string());
//         assert!(res.is_err());
//         assert_eq!(
//             res.unwrap_err().to_string(),
//             r#"illegal list value: "a-1""#,
//         );

//         // Wonky ranges
//         let res = parse_pos("-".to_string());
//         assert!(res.is_err());

//         let res = parse_pos(",".to_string());
//         assert!(res.is_err());

//         let res = parse_pos("1,".to_string());
//         assert!(res.is_err());

//         let res = parse_pos("1-".to_string());
//         assert!(res.is_err());

//         let res = parse_pos("1-1-1".to_string());
//         assert!(res.is_err());

//         let res = parse_pos("1-1-a".to_string());
//         assert!(res.is_err());

//         // First number must be less than second
//         let res = parse_pos("1-1".to_string());
//         assert!(res.is_err());
//         assert_eq!(
//             res.unwrap_err().to_string(),
//             "First number in range (1) must be lower than second number (1)"
//         );

//         let res = parse_pos("2-1".to_string());
//         assert!(res.is_err());
//         assert_eq!(
//             res.unwrap_err().to_string(),
//             "First number in range (2) must be lower than second number (1)"
//         );

//         // All the following are acceptable
//         let res = parse_pos("1".to_string());
//         assert!(res.is_ok());
//         assert_eq!(res.unwrap(), vec![0..1]);

//         let res = parse_pos("01".to_string());
//         assert!(res.is_ok());
//         assert_eq!(res.unwrap(), vec![0..1]);

//         let res = parse_pos("1,3".to_string());
//         assert!(res.is_ok());
//         assert_eq!(res.unwrap(), vec![0..1, 2..3]);

//         let res = parse_pos("001,0003".to_string());
//         assert!(res.is_ok());
//         assert_eq!(res.unwrap(), vec![0..1, 2..3]);

//         let res = parse_pos("1-3".to_string());
//         assert!(res.is_ok());
//         assert_eq!(res.unwrap(), vec![0..3]);

//         let res = parse_pos("0001-03".to_string());
//         assert!(res.is_ok());
//         assert_eq!(res.unwrap(), vec![0..3]);

//         let res = parse_pos("1,7,3-5".to_string());
//         assert!(res.is_ok());
//         assert_eq!(res.unwrap(), vec![0..1, 6..7, 2..5]);

//         let res = parse_pos("15,19-20".to_string());
//         assert!(res.is_ok());
//         assert_eq!(res.unwrap(), vec![14..15, 18..20]);
//     }

//     #[test]
//     fn test_extract_fields() {
//         let rec = StringRecord::from(vec!["Captain", "Sham", "12345"]);
//         assert_eq!(extract_fields(&rec, &[0..1]), &["Captain"]);
//         assert_eq!(extract_fields(&rec, &[1..2]), &["Sham"]);
//         assert_eq!(
//             extract_fields(&rec, &[0..1, 2..3]),
//             &["Captain", "12345"]
//         );
//         assert_eq!(extract_fields(&rec, &[0..1, 3..4]), &["Captain"]);
//         assert_eq!(extract_fields(&rec, &[1..2, 0..1]), &["Sham", "Captain"]);
//     }

//     #[test]
//     fn test_extract_chars() {
//         assert_eq!(extract_chars("", &[0..1]), "".to_string());
//         assert_eq!(extract_chars("ábc", &[0..1]), "á".to_string());
//         assert_eq!(extract_chars("ábc", &[0..1, 2..3]), "ác".to_string());
//         assert_eq!(extract_chars("ábc", &[0..3]), "ábc".to_string());
//         assert_eq!(extract_chars("ábc", &[2..3, 1..2]), "cb".to_string());
//         assert_eq!(
//             extract_chars("ábc", &[0..1, 1..2, 4..5]),
//             "áb".to_string()
//         );
//     }

//     #[test]
//     fn test_extract_bytes() {
//         assert_eq!(extract_bytes("ábc", &[0..1]), "�".to_string());
//         assert_eq!(extract_bytes("ábc", &[0..2]), "á".to_string());
//         assert_eq!(extract_bytes("ábc", &[0..3]), "áb".to_string());
//         assert_eq!(extract_bytes("ábc", &[0..4]), "ábc".to_string());
//         assert_eq!(extract_bytes("ábc", &[3..4, 2..3]), "cb".to_string());
//         assert_eq!(extract_bytes("ábc", &[0..2, 5..6]), "á".to_string());
//     }
// }
