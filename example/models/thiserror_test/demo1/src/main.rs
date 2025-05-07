use clap::Parser;
use err_test::core::process_files;

/// Simple adder
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// input file path
    #[arg(short, long)]
    input: String,
}

fn main() {
    let args = Args::parse();
    let input_path = args.input;
    println!("input: {}", input_path);
    // process_files(&input_path); //最开始没有返回值
    let _result_enum = process_files(&input_path);
    println!("{:?}",_result_enum);
    /*
    pub enum Result<T, E> {
        Ok(T),
        Err(E),
    }
     */

    // 匹配result 类型，自定义错误无法处理
    // match process_files(&input_path) {
    //     Ok(_) => {}
    //     Err(e) => println!("Error: {}", e),
    // }
}