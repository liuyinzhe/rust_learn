/*
    thiserror
        为std::err:Error 提供了方便的宏
    anyhow
        提供了anyhow::Error
        可在Rust应用程序中轻松进行惯用错误处理

*/
use thiserror::Error;
use anyhow::Result;
#[derive(Debug,Error)]
enum Error{
    #[error(transparent)]
    VarError(#[from] std::env::VarError),
    //#[error("parse error: {0}, please check IO")]
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}


// fn main() -> Result<(),Error>{ //thiserror::Error // Error: VarError(NotPresent)
fn main() -> Result<()>{// anyhow::Result // Error: environment variable not found
    let s = std::env::var("DIR_PATH")?;
    let f: String = std::fs::read_to_string("path")?;
    println!("{},{}",s,f);
    Ok(())
}
