fn main() {
    if let Err(e) = catr::get_args().and_then(catr::run){ // 将get_args 返回值配置 Config 结构体 作为参数传递给 catr::run()
        eprintln!("{}",e); // 错误打印行, 打印到STDERR
        std::process::exit(1); // 设置非0退出
    }
}
// cargo add  clap --features derive
// cargo add rand