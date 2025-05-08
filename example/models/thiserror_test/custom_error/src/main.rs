
/*
    std::err:Error 是一个特质
    pub trait Error:Debug + Display {
        fn source(&self)-> Option<&(dyn Error + 'static)> {None}
    }
    // 不重写source的情况
    // 重写source的情况
*/

// use std::error::Error;



#[derive(Debug)]
// struct ErrorA{
//     err:ErrorB // 自定的错误
// }
struct ErrorA;


impl std::fmt::Display for ErrorA { // trail 共享
    fn fmt(&self,f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        write!(f, "ErrorA")// write! 打印 "ErrorA"
    }
}

impl std::error::Error for ErrorA{ // trail 共享
    // 不重写source函数(fn),直接继承
    // 重新写
    // fn source(&self)-> Option<&(dyn std::error::Error + 'static)> {
    //     Some(&self.err) // 来自ErrorA 结构体中的 err:ErrorB
    // }
}

fn new_error_a() -> Result<(),ErrorA> {
    Err(ErrorA)
}

// 从ErrorB转换为ErrorA
impl From<ErrorB> for ErrorA{
    fn from(_: ErrorB) -> Self{
        ErrorA
    }
}

////////// B
#[derive(Debug)]
struct ErrorB;

impl std::fmt::Display for ErrorB { // trail 共享
    fn fmt(&self,f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        write!(f, "ErrorB")// write! 打印 "ErrorB
    }
}

impl std::error::Error for ErrorB{ // trail 共享
    // 不重写函数(fn),直接继承
}

fn new_error_b() -> Result<(),ErrorB> {
    Err(ErrorB)
}

// 从ErrorA转换为ErrorB
impl From<ErrorA> for ErrorB{
    fn from(_: ErrorA) -> Self{
        ErrorB
    }
}

// fn main() -> Result<(),Box<dyn std::error::Error>> {
fn main() -> Result<(),ErrorA> {
    // match new_error_a(){// ? 解析Result类型;// 当前用match Result类型
    //     Err(e) => {
    //         println!("Error:{}",e); // Error:ErrorA
    //         println!("Caused by:{}",e.source().ok_or(ErrorB)?)//ErrorB//Option 使用ok_or()转为了Result 枚举类型
    //         //.unwrap()) // Caused by:ErrorB
    //     }
    //     _ => println!("OK")
    // }
    new_error_b()?;
    println!("继续");
    Ok(())
}
