use clap::{Command,Arg,ArgAction}; // App 重命名为 Command
// cargo add clap

fn main() {
    // println!("{:?}",std::env::args()); //  cargo run -- -n xx yy 
    // // Args { inner: ["target/debug/echor", "-n", "xx", "yy"] }

    let matches = Command::new("echor") // 新建结构体
        .version("0.1.0") 
        .author("author name<author@email.com>")
        .about("Rust echo")
        .arg(
            Arg::new("text") // with_name 替换为了 new
            .value_name("TEXT")
            .help("Input Text")
            .required(true) // 必须有这个参参数
            .num_args(1..) //.min_values(1) // 出现次数至少一次
        )
        .arg(
            Arg::new("omit_newline")
            .short('n') // 短名字 ,接收字符(char), 而不是字符串(String),'n'(单引号,表示字符) "n"(双引号,表示字符串)
            .help("Do not print newline")
            .action(ArgAction::SetTrue)//.takes_value(false) // 不接收具体值
        )
        .get_matches(); // 解析参数
    // println!("{:#?}",matches); // :#? 格式化打印


    // let text = matches.values_of_lossy("text").unwrap();
    // 替代 values_of_lossy 的方法
    // let text: Vec<String> = matches
    //     .get_many::<String>("text") // Option<ValuesRef<'_, String>>
    //     .unwrap() // ValuesRef<'_, String> 
    //     .map(ToString::to_string) // Map<..., fn(&String) -> String>
    //     // ToString是一个trait
    //     .collect(); // Vec<String>

    let text: Vec<String> = matches
        .get_many("text")           // Option<ValuesRef<'_, String>>
        .expect("text is required") // ValuesRef<'_, String> (如果None则panic) 与unwrap类似,但如果结果是None,则会panic并显示指定的错误信息"text is required",有助于调试。
        .cloned()                   // 创建一个新的迭代器,其元素类型为String(通过克隆每个元素);
        // 迭代器的元素类型是&String(字符串的引用)
        // String实现了Clone trait
        // 克隆，从而将&String转换为String
        .collect();                 // 将迭代器收集为Vec<String>
       


    // let omit_newline= matches.is_present("omit_newline");
    let omit_newline = matches.get_flag("omit_newline");

    let ending = if omit_newline {" "} else { "\n"};
    print!("{}{}",text.join(" "),ending); // Vec<String>  Vec.join()

}

// Rust echo

// Usage: echor [OPTIONS] <TEXT>...

// Arguments:
//   <TEXT>...  Input Text

// Options:
//   -n             Do not print newline
//   -h, --help     Print help
//   -V, --version  Print version