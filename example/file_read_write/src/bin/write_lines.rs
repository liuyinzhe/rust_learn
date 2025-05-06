
use std::{fs::File, io::Write, env::args};
fn main(){
    //todo!();

    // <Vec<_>>显式指定目标类型为Vec(动态数组)，_ 表示自动推导元素类型
    let args = args().collect::<Vec<_>>(); // collect() -> vecter
    // parse::<usize>()将字符串解析为指定数值类型
    // 泛型‌：::<usize>显式声明目标类型，避免类型推导歧义
    // unwrap() 解包Result类型，若为Ok则返回值，若为Err则触发panic终止程序
    let n = args[1].parse::<usize>().unwrap();
    println!("will write {} lines",n);

    let mut file = File::create("./data/a.txt").unwrap();// 相对crate root 目录
    for line in Lines::new().take(n){ // take(10)取固定行
        //println!("{}",line);//额外补充换行
        //print!("{}",line);

        //write_all 方法写入字符串字节流(as_bytes)
        //采用unwrap()处理I/O错误
        file.write_all(line.as_bytes()).unwrap();
    }
    println!("done");
}

// 元组结构体 (Tuple Struct)
// struct Color(i32, i32, i32);
struct Lines(usize); //创建内容类型为整数 usize类型的结构体

impl Lines{ // Line 创建new方法
    fn new() -> Lines{
        Lines(0) // 返回一个内容为 usize类型,数值为0的结构体
    }
}

// 迭代器 Iterator trait 共享给Lines 结构体
impl Iterator for Lines{
    // ‌契约约束‌：实现 Iterator trait 的必须声明 Item
    type Item = String; // 声明迭代元素类型
    // 重写next 方法
    fn next(&mut self) -> Option<String>{ // 编译器检查 next() 返回值是否匹配 Option<Self::Item>
        let n: usize = 10000000000 + self.0; // 元组结构体取第一个数值(结构体只有单一元素)
        let line:String = format!("{}\n",n); // 数字format连接为字符串
        self.0 += 1; // 元组结构体第一个元素累加1
        Some(line) // 返回line 字符串
    }
}