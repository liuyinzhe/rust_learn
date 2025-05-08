
/*
    可恢复的错误    Result<T,E>
    不可恢复的错误  panic!

    enum Result<T,E>{
    Ok(T),
    Err(E),
    }
    // T/U/E 泛型

    unwarp() 可以打开 Option和Result的封装
        如果Err或是None直接panic!
        然后程序崩溃
    ? 是语法糖
        如果是Err就返回Err类型
        Option ok_or(err:Error); Option 推荐用match
*/
//fn main(){
fn main() -> Result<(),std::io::Error> {
    let s  = std::fs::read_to_string("text.txt")?;//.unwrap();
    println!("{}",s);
    Ok(())
}
