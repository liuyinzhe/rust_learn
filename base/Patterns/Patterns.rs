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
    */


    /*
        模式的语法
        Pattern Syntax
    */

    // 匹配字面量
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 匹配命名变量
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),\
        // 重要：此处的 y 是新建的局部变量(遮蔽外部 y)，绑定 Some 内部的值 y=x 遮蔽作用域只在{}内
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {x:?}"),
    }
    // 离开 match 作用域后，新建的 y 失效
    println!("at the end: x = {x:?}, y = {y}");

    // 多模式匹配
    let x = 1;
    
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _  => println!("anything"),
    }

    // 匹配范围值 ..=
    let x = 5;
    match x {
        1..=5 => println!("one throught five"),
        _ => println!("something else"),    
    }

    let x = 'c';
    match x {
        'a'..='j' =>println!("early ASCII letter"),
        'k'..='z' =>println!("late ASCII letter"),
        _ => println!("something else"),
    }


    // 解构树类型
    // 解构 Struct

    struct Point {
        x:i32,
        y:i32,
    }

    fn main() {
        let p = Point {x:0, y:7};
        // 解构
        let Point {x:a, y:b } = p; // x,y 字段名提取
        assert_eq!(0,a);
        assert_eq!(7,b);
    }

    struct Point {
        x: i32,
        y: i32,
    }
    
    fn main() {
        
        let p = Point {x:0, y:7};
        // 解构
        let Point {x, y} = p; // x,y 形参提取
        assert_eq!(0,x);
        assert_eq!(7,y);
    }

    fn main() {
        let p = Point {x: 0, y:7};
        
        match p {
            // y 指定数值
            Point { x, y:0} => println!("On the x axis at {x}");
            // x 指定数值
            Point {x:0, y } => println!("On the y axis at {y}");
            // x/y 内容任意| 其它
            Point {x,y} => {
                println!("On neither axis: ({x},{y})");
            }
        }
    }

    // 解构 Enum 枚举

    enum Message {
        Quit,
        Move {x: i32, y: i32},
        Write(String),
        ChangeColor(i32,i32,i32),
    }

    fn main() {
        let msg = Message::ChangeColor(0,160,255);

        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.");
            }
            Message::Move {x,y} => {
                println!("Move in the x direction {x} and in the y direction {y}");
            }
            Message::Write(text) => { // 作用内形式参数/变量,被赋值
                println!("Text message: {text}");
            }
            Message::ChangeColor(r,g,b) => {
                println!("Change the color to red {r},green {g}, and blue {b}");
            }

        }
    }


    // 解构 嵌套解构

    enum Color {
        Rgb(i32,i32,i32),
        Hsv(i32,i32,i32),
    }

    enum Message {
        Quit,
        Move {x: i32, y:i32},
        Write(String),
        ChangeColor(Color),
    }

    fn main() {
        // 解构Color 变体
        let msg = Message::ChangeColor(Color::Hsv(0,160,255));

        match msg {
            Message::ChangeColor(Color::Rgb(r,g,b)) => { // r g b 对应msg数值
                println!("Change color to red {r}, green {g}, and blue {b}");
            }
            Message::ChangeColor(Color::Hsv(h,s,v)) => {
                println!("Change color to hue {h}, saturation {s}, value {v}");
            }
            _ =>(),
        }
    }

    // 解构 Structs 和 Tuples

    let ((feet, inches),Point{x,y}) = ((3,10),Point {x:3,y:-10});

    // 忽略模式中的值
    // 使用 _ 忽略整个值

    fn foo(_:i32,y:i32) {
        println!("This code only uses the y parameter: {y}");
    }
    
    fn main() {
        foo(3,4);
    }

    // 使用嵌套的 _ 忽略部分值

    let mut setting_value = Some(5);
    let new_setting_value= Some(10);

    match (setting_value,new_setting_value) {
        (Some(_),Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {setting_value:?}");


    let numbers = (2,4,8,16,32);

    match numbers {
        (first,_,third,_,fifth) => {
            println!("Some numbers: {first},{third},{fifth}");
        }
    }


    // 使用以 _ 开头的名字，忽略未(不)使用的变量

    fn main() {
        let _x = 5;
        let y = 10; // 警告未使用
    } 

    let s = Some(String::from("Hello!"));

    if let Some(_s) = s {
        println!("found a string");
    }
    println!("{s:?}");

    let s = Some(String:from("Hello!"));
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{s:?}");

    // 使用 .. 忽略剩余部分

    Struct Point {
        x:i32,
        y:i32,
        z:i32,
    }

    let origin = Point {x:0, y:0, z:0};

    match origin {
        // .. 忽略其它解构体字段
        Point {x, ..} => println!("x is {x}");
    }


    fn main() {
        let numbers = (2,4,8,16,32);

        match numbers {
            (first, .., last) => {
                println!("Some numbers: {first},{last}");
            }
        }
    }


    fn main() {
        let numbers = (2,4,8,16,32);

        match numbers {
            (.., second,..) => { // Error 位置编译器无法确定
                println!("Some numbers: {second}");
            }
        }
    }


    // 匹配守卫
    // Extra Conditionals with Match Guards

    let num = Some(4);

    match num {
        // 匹配守卫: x 能被2 整除
        Some(x) if x % 2 == 0 ==> println!("The number {} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }


    fn main() {
        let x = Some(5);
        let y = 10;

        match  x {
            Some(50) => println!("Got 50"),
            Some(n) if n == y => println!("Matched,n={n}"),
            _ => println!("Default case, x = {x:?}"),
        }
        println!("at the end: x = {x:?}, y = {y}");
    }

    
    let x= 4;
    let y = false;

    match x {
        // 解读
        // (4|5|6) if y => ...
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // @ 绑定
    // @ Bindings

    enum Message {
        Hello {id: i32},
    }

    let msg = Message::Hello {id:5};

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7, // 3-7范围内，则数值id_variable绑定到id 变量中
        } => println!("Found an id in range: {id_variable}"),
        // 在 10-12 范围内
        Message::Hello {id: 10..=12} => {
            println!("Found an id in another range")
        } // 作用域 不用写逗号,
        Message::Hello {id} => println!("Found some other id: {id}"),
    }
