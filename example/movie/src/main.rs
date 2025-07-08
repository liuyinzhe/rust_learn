use clap::{Parser,Subcommand};
use movie::handler::{handle_loging,handle_logout};

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

#[derive(Subcommand)]
enum Commands {
  /// User log into the system
  Login {
    /// The username of the user
    #[arg(short,long)]
    username: String
  },
  /// Log out
  Logout,
    
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let cli = Cli::parse();
    // println!("{}",cli.name);
    let cli = Cli::parse();
    match &cli.commands{
        Some(Commands::Login {username}) => handle_loging(username)?,
        Some(Commands::Logout) => handle_logout(),
        _ => println!("No command provider or command not recognized"),
      }
      Ok(())
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