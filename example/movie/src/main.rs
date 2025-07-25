use clap::{Parser,Subcommand};
use movie::handler::{handle_loging,handle_logout,handle_list,handle_add,handle_delete,handle_edit};

#[derive(Parser)]
#[command(
    version,
    about = "Movie app", // -h
    long_about = "Movie infomation app" // --help
)]
struct Cli {
  // // 加参数
  // name: String,// 对应第一个参数,类似args[1]
  // #[arg(short,long)] // -a --age
  // age: usize, // cargo run -- aa  -a 18
  #[command(subcommand)]
  commands: Option<Commands>

}

#[derive(Subcommand)] // 子命令
enum Commands { // 子命令-枚举
  /// User log into the system
  Login {
    /// The username of the user
    #[arg(short,long)] // 长短参数
    username: String
  },
  /// Log out
  Logout,
  /// list all the movies
  List,
  /// Add a movie
  Add {
    /// The disc no. of the movie
    #[arg(short,long)]
    disc: usize,

    /// The year when the movie was released
    #[arg(short,long)]
    year: String,

    /// The title / file name of movie
    #[arg(short,long)]
    title: String,

    // Optional remark of the movie
    #[arg(short,long)]
    remark: Option<String>
  },
  /// Delete a movie
  Delete {
    /// The disc no. of the movie
    #[arg(short,long)]
    disc: usize,

    /// The index of the movie in the disc
    #[arg(short,long)]
    index: usize
  },

  /// Modify a movie
  Edit {
    /// The disc no. of the movie
    #[arg(short,long)]
    disc: usize,

    /// The index of the movie in the disc
    #[arg(short,long)]
    index: usize
  }
}
/*cargo run -- --help
Movie infomation app

Usage: movie.exe [COMMAND]

Commands:
  login  User log into the system
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

*/
/* cargo run -- loging --help
Usage: movie.exe login --username <USERNAME>

Options:
  -u, --username <USERNAME>  The username of the user
  -h, --help                 Print help
*/

fn main() -> Result<(), Box<dyn std::error::Error>> { // 动态错误类型，兼容任意实现了 Error trait 的错误
    // let cli = Cli::parse();
    // println!("{}",cli.name);
    let cli = Cli::parse();
    match &cli.commands{  // Option match Some 内容或者 None
        // cargo run -- login --username admin
        Some(Commands::Login {username}) => handle_loging(username)?,
        Some(Commands::Logout) => handle_logout(),
        // cargo run -- list
        Some(Commands::List) => handle_list()?, // Result
        // cargo run -- add --disc 150 --year 2025 --title "Some"
        Some(Commands::Add { 
          disc, 
          year, 
          title, 
          // handle_add(disc: usize, year: &str, title: &str, remark: &Option<String>)
          // 由于 形参 disc 是实际数字, 实际参数disc 是一个引用指针,需要使用*解引用
          remark 
        }) => handle_add(*disc,year,title,remark)?, // 返回Result
        // cargo run -- delete --disc 150 --index 0
        Some(Commands::Delete { disc, index }) => handle_delete(disc,index)?,
        // cargo run -- edit --disc 122 --index 0 // 150 // 
        Some(Commands::Edit { disc, index }) => handle_edit(disc,index)?,
        _ => println!("No command provider or command not recognized"),
      }
      Ok(()) // 空返回
}

// cargo add clap -F derive
// cargo run -- 

// target\debug\movie.exe --version
// movie 0.1.0


// cargo run -- --help
/*
Movie infomation app

Usage: movie.exe

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
*/

// cargo run -- -h
/*
Movie app

Usage: movie.exe

Options:
  -h, --help     Print help (see more with '--help')
  -V, --version  Print version
*/


// cargo build --release