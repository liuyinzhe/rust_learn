
// cargo add thiserror
// cargo run --example example11 // 错误处理(3)
/*
    自定义错误类型
    定义一个Struct 或 enum
        实现 Error trait
        实现 Display trait

    thiserror crate
*/

use thiserror::Error;
use std::fs;

fn main() {
    println!("{:?}", load_books());
}
#[derive(Debug,Error)] // 实现 Debug trait 和 thiserror的Error trait
// #[derive(Debug,Clone)]
enum BooksError { // 自定义错误
    #[error("Book not found")] // thiserror crate 提供的宏，用于自动生成错误实现的代码
    BookNotFound,
    #[error("Too many book")]
    TooManyBook,

    #[error("Failed to read book's file")]
    FileReadFailed,
}

// impl std::fmt::Display for BooksError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::BookNotFound => write!(f, "Book not found"),
//             Self::TooManyBook => write!(f, "Too many book"),
//         }
//     }
// }

// Result<T,E> // Ok(),Err()
fn load_books() -> Result<Vec<String>,BooksError> {
    let content = fs::read_to_string("books.txt")
    .map_err(|_| BooksError::FileReadFailed)?; // 映射err 将Err转换为BooksError// ? 取出Ok中的值
    
    let books:Vec<String> = content
        .lines()  // 返回字符串的行迭代器，每行包含换行符但已去除 \n 或 \r\n
        .map(|s| s.trim()) //去除每行首尾的空白字符（空格、制表符等）
        .filter(|s| !s.is_empty()) // 过滤掉空行（即去除空白后长度为0的行）。
        .map(String::from) //将 &str 转换为拥有所有权的 String
        .collect(); // 将迭代器收集为 Vec<String>，类型由左侧变量 books: Vec<String> 推断

    if books.is_empty() {
        Err(BooksError::BookNotFound)
    } else if books.len() > 5 {
        Err(BooksError::TooManyBook)
    } else {
        Ok(books)
    }


}
