// std::error::Error: Rust 标准库中的错误 trait，用于统一错误处理
// std::process进程控制模块(用于程序退出)
use std::{error::Error, process};
// rfd::FileDialog: 跨平台文件对话框库
use rfd::FileDialog;
use directories::UserDirs; // cargo add directories // 当前用户桌面(跨平台)
use move_importer::read_txt_to_json;

fn main() -> Result<(),Box<dyn Error>> { // Box<dyn Error>: 动态分发的错误类(可包含任何实现了 Error trait 的类型)
    // match Some() None
    // Some 赋值 非None 返回值
    match FileDialog::new().add_filter("Text Files",&["txt"]) // 返回 Option<T>,使用match
    .set_title("Select the DVD text file")
    //.set_directory("/Users/dave/Desktop")
    .set_directory(UserDirs::new().unwrap().desktop_dir().unwrap())
    .pick_file(){
        Some(file_path) => {
            let saved_path = read_txt_to_json(&file_path)?;
            println!("Data saved to: {saved_path:?}");
            Ok(())
        }
        None => {
            eprintln!("File not selected.");
            process::exit(-1)
        }
    }
}


// 发布
// cargo build --release
