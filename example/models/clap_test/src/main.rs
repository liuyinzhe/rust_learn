

// // 使用 command 构建解析器
// use clap::{arg, Command};
// fn main() {
    
//     let matches = Command::new("MyApp")
//         .version("1.0")
//         .author("Kevin K. <kbknapp@gmail.com>")
//         .about("Does awesome things")
//         .arg(arg!(--one <VALUE>).required(true))
//         .arg(arg!(--two <VALUE>).required(true)) // 参数顺序与这里有关
//         .get_matches();

//         // 参数顺序与这里无关
//         println!(
//             "two: {:?}",
//             matches.get_one::<String>("two").expect("required")
//         );

//         println!(
//             "one: {:?}",
//             matches.get_one::<String>("one").expect("required")
//         );

//         if let Some(param) = matches.get_one::<String>("one") {
//             println!("输入的参数是: {}", param);
//         }
// }

// // type2
use clap::{arg, Command};
fn main() {
    let matches = Command::new("myapp")
  			.version("1.0.0")
  			.author("hedon")
  			.about("this is the short about")
  			.long_about("this is the long about")
        .arg(arg!([NAME]).required(true).help("Specify your name"))
        .arg(arg!(-a --age <AGE>)
            .value_parser(clap::value_parser!(u8))
            .help("Specify your age optionally"))
        .get_matches();

    println!("name: {:?}", matches.get_one::<String>("NAME"));
    println!("age: {:?}", matches.get_one::<u8>("age"));
}

