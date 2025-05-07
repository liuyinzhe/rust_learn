use std::fs::File;
use std::io::{BufRead, BufReader};

use thiserror::Error; // 引入宏 // 方法4

#[derive(Error, Debug)]
pub enum MyError {
    #[error("io error: {0}, please check file")] //#[error()] 宏用来定义错误信息,成员中的#[from] 宏定义了错误类型的转换
    Io(#[from] std::io::Error),
    #[error("parse error: {0}, please check content")]
    Parse(#[from] std::num::ParseIntError),
    // Other error
    #[error(transparent)]
    Other(#[from] anyhow::Error), //.ok_or(MyError::Other(anyhow::anyhow!("not excepted")))?
}

// use std::error::Error; // 方法1:用trait对象传递错误

// #[derive(Debug)]
// // 手动自定义错误
// pub enum MyError {
//     IO(std::io::Error),
//     Parse(std::num::ParseIntError),
// }
// // 手动自定义错误 实现From这个trait
// impl From<std::io::Error> for MyError { // From 约束<std::io::Error>
//     fn from(err: std::io::Error) -> Self {
//         MyError::IO(err)
//     }
// }
// // 手动自定义错误 实现From这个trait
// impl From<std::num::ParseIntError> for MyError { // From 约束<std::num::ParseIntError>
//     fn from(err: std::num::ParseIntError) -> Self {
//         MyError::Parse(err)
//     }
// }

// 为 MyError 实现 Display trait
// impl std::fmt::Display for MyError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { // Formatter<'_>自动推断生命周期, Formatter<'a>	显式生命周期参数
//         match self { // 判断是不是,返回值
//             MyError::IO(e) => write!(f, "IO错误: {}", e),
//             MyError::Parse(e) => write!(f, "解析错误: {}", e),
//         }
//     }
// }

// impl std::fmt::Display for MyError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             MyError::IO(err) => write!(f, "IO error: {}, please check file", err),
//             MyError::Parse(err) => write!(f, "Parse error: {}, please check content", err),
//         }
//     }
// }

//pub fn process_files(input_path: &str) { // 最开始没有返回值
//pub fn process_files(input_path: &str) -> Result<(), Box<dyn Error>> { // 方法1:用trait对象传递错误
//pub fn process_files(input_path: &str) -> Result<(), MyError> { // 方法2:自定义错误
//pub fn process_files(input_path: &str) -> anyhow::Result<()> { // 方法3:anyhow
pub fn process_files(input_path: &str) -> Result<(), MyError> { // 方法4:thiserror;实现各种繁琐的特征实现,由过程宏来简化
    // Open the input file
    let input_file = File::open(input_path)?;//.unwrap();//全都要改?;
    let reader = BufReader::new(input_file);

    // Accumulator for the sum
    let mut sum = 0;

    // Read each line from the input file and parse it as i32
    for line in reader.lines() {
        let line = line?;//.unwrap();//全都要改?;
        let number: i32 = line.parse()?;//.unwrap();//全都要改?;
        sum += number;
    }

    // Write the sum to the output file
    println!("sum: {}", sum);
    Ok(()) // 最开始没有返回值
}