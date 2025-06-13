/*
    能使用模式(匹配)的地方
    All the Places Patterns Can Be Used

    模式
    Pattern

    Rust 中的特殊语法，用于匹配简单或复杂类型结构
        例如: 与match 表达式和其他结构结合，增强程序控制流程

    模式的组成

    字面值(Literals):例如数字或字符串
    解构数据结构: 数组、枚举、结构体、元组等
    变量(Variables):命令的变量
    通配符(Wildcards):_表示人一直
    占位符(Placeholders):尚未具体定义的部分

    使用方法

    比较模式与值,若匹配成功则提取并使用数据
    常用场景:例如match 表达式,允许根据数据形状选择不同的代码路径

    关键点
    可反驳模式 vs 不可反驳模式
    学会运用模式以清晰地表达编程概念

*/


    // 可以使用模式的地方
    // match 表达式 (可穷尽类型的类型)

    match x {
        None => None,
        Some(i) => Some(i+1),
    }

    // 条件if let 表达式
    if let Some(color) = favorite_color {
        println!("xxx");
    } else if is_tuesday {
        println!("yyy");
    }

    // while let 条件循环

    let (tx,rx) = std::sync::mpsc::chanel();
    std::thread::spawn(move|| {
        for val in [1,2,3] {
            tx.send(val).unwrap();
        }
    });

    while let Ok(value) = rx.recv() {
        println!("{value}");
    }

    // for 循环
    let v = vec!['a','b','c'];

    for (index,value) in v.iter().enumerate(){
        println!("{value} is at index {index}");
    }

    // let 语句
    let PATTERN = EXPRESSION;

    let x = 5
    let (x,y,z) = (1,2,3);
    let (x,y) = (1,2,3);// error

    // 函数的参数
    fn foo(x: i32){
        // code goes here
    }

    fn print_coordinates(&(x,y):&(i32,i32)) {
        println!("Current location: ({x}, {y})");
    }
    
    fn main() {
        let point = (3,5);
        print_coordinates(&point);
    }

    /*
        Rust模式中的可反驳行 
        Refutability: Whether a Pattern Might Fail to Match

        不可反驳模式: 适用于所有可能的值,例如let x=5 中的x。(不可能匹配不上)
        可反驳模式: 可能不匹配某些值,例如 if let Some(x) = a_value 中的Some(x)。(可能匹配不上)
    /
