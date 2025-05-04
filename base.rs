
fn main(){
    let mut x = 5;
    println!("The value of x is {}",x);
    x=6;
    println!("The value of x is {}",x);

    // 变量解构 
    let (a, mut b): (bool,bool) = (true, false);
    // a = true, 不可变; b = false, 可变
    println!("a = {:?}, b = {:?}", a, b);
    b = true;
    assert_eq!(a, b);

    // 常量
    const MAX_POINTS: u32 = 100_000;

    // 变量遮蔽
    let x = 5;
    // 在 main 函数的作用域内对之前的 x 进行遮蔽
    let x = x + 1;

    /*
    长度	有符号类型	无符号类型
    8	i8	u8
    16	i16	u16
    32	i32	u32
    64	i64	u64
    128	i128	u128
    视架构而定	isize	usize
    */
    
    //panic(崩溃,Rust 使用这个术语来表明程序因错误而退出)
    
    // 整数溢出
    /*
    要显式处理可能的溢出, 可以使用标准库针对原始数字类型提供的这些方法：

    使用 wrapping_* 方法在所有模式下都按照补码循环溢出规则处理, 例如 wrapping_add
    如果使用 checked_* 方法时发生溢出, 则返回 None 值
    使用 overflowing_* 方法返回该值和一个指示是否存在溢出的布尔值
    使用 saturating_* 方法使值达到最小值或最大值
     */

    // NaN 
    // (Not a Number)值。可以使用 f32::NAN 或 f64::NAN 来创建一个 NaN 值。NaN 值不等于任何值, 包括它自己。
    // NaN 值也不小于、不大于或等于任何值。
     let x = (-42.0_f32).sqrt();
     if x.is_nan() {
         println!(" 未定义的数学行为 ")
     }

     /*
    数字运算
    Rust 支持数字类型的基本数学运算：

    加法：+
    减法：-
    乘法：*
    除法：/
    取余：%
    支持位运算：

    按位与：&：相同位置均为 1 时则为 1, 否则为 0
    按位或：|：相同位置有一个为 1 时则为 1, 否则为 0
    按位异或：^：相同位置不同时则为 1, 否则为 0
    按位取反：!：相同位置取反
    左移：<<：将二进制数向左移动指定位数, 右位补 0
    右移：>>：将二进制数向右移动指定位数, 左位补 0
      */
    
    // 范围 1-4
    for i in 1..5 {
        println!("{}", i);
    }
    // 1-5
    for i in 1..=5 {
        println!("{}", i);
    }
    // 字符
    for i in 'a'..='e' {
        println!("{}", i);
    }

    

}

// 函数就是 [表达式](# 语句和表达式), 函数的返回值就是函数体最后一条表达式的返回值, 也可以用 return 关键字提前返回。
fn plus_or_minus(x:i32) -> i32 {
    if x > 5 {
        return x - 5
    }

    x + 5
}


// 拷贝特性

let x = 5;
let y = x;

// 没有拷贝 所有权转移 指针修改(根本原因是存在堆上)
let s1 = String::from("hello");
let s2 = s1;


///////

fn main() {
    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里 ...
                                    // ... 所以到这里不再有效

    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 移动函数里, 
                                    // 但 i32 是 Copy 的, 所以在后面可继续使用 x

} // 这里, x 先移出了作用域, 然后是 s。但因为 s 的值已被移走, 
  // 所以不会有特殊操作

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里, some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里, some_integer 移出作用域。不会有特殊操作


// 可变引用和不可变引用不能同时存在
fn main() {
    let mut s = String::from("hello");
    
    let r1 = &s; // 创建第一个引用, 不出错
    let r2 = &s; // 创建第二个引用, 允许, 不出错
    let r3 = &mut s; // 创建一个可变引用, 不允许, 出错
    
    println!("{}, {}, and {}", r1, r2, r3);
    }
/// 悬垂引用 (Dangling References)

fn main() {
    let reference_to_nothing = dangle();  // 报错咯, 因为这里返回的引用, 指向了一个不存在的值
}

fn dangle() -> &String {
    let s = String::from("hello"); // 创建一个字符串 String

    // &s // 返回 s 的引用
    s  // 返回所有权而非引用
}

// 参数, 生命周期 函数生命周期, 变量生命周期, 返回值生命周期
fn dangle<'a>(s: &'a mut String) -> &'a str {
    s.push_str(", world");
    &s[..]
}

// 字符串切片
// 分别引用了 s 的部分内容, 通过 [0..5] 和 [6..11] 来指定。这种语法用来创建 slice,方括号左边界闭区间,右边界开区间。
fn main() {
    let s = String::from("hello world");
    
    let hello = &s[0..5];
    let world = &s[6..11];
    // 边界可以不写
    // let slice = &s[0..2];
    // let slice = &s[..2];

    // let len = s.len();
    // let slice = &s[4..len];
    // let slice = &s[4..];

    }

// 字符串类型
// Rust 在语言级别,只有一种字符串类型:str,通常以引用方式出现:&str
fn main() {
    let s: &str = "Hello, world!";
    }
// 堆上分配一块在编译时未知大小的内存来存放内容
// str 类型被编译成了固定大小的字符串,长度不可变,也无法被修改;
// String 类型是可变的

// string str 转换
// to_string() 方法 
let s = "hello".to_string();
// String::from() 方法
let s = String::from("hello");

fn main() {
    let s = String::from("hello,world!");
    say_hello(&s);
    say_hello(&s[..]);
    say_hello(s.as_str()); // as_str() 方法, 就是 Extracts a string slice containing the entire String.

}

fn say_hello(s: &str) {
    println!("{}",s);
}

// String 类型无法索引
fn main() {
    let s1 = String::from("hello");
    let h = s1[0];// 报错,因为String 是Vec<u8>封装
}
// 但是String 可以被修改
fn main() {
    let mut s = String::from("Hello ");  // mut 关键字声明可变
    s.push('r'); // push() 方法, 追加单个字符
    println!(" 追加字符 push() -> {}", s);

    s.push_str("ust!"); // push_str() 方法, 追加字符串
    println!(" 追加字符串 push_str() -> {}", s);
}

fn main() {
    let mut s = String::from("Hello rust!");
    s.insert(5, ',');
    println!(" 插入字符 insert() -> {}", s);
    s.insert_str(6, " I like");
    println!(" 插入字符串 insert_str() -> {}", s);
}

// 替换(Replace) 支持 String &str
fn main() {
    let string_replace = String::from("I like rust. Learning rust is my favorite!");
    let new_string_replace = string_replace.replace("rust", "RUST");
    dbg!(new_string_replace);  // "I like RUST. Learning RUST is my favorite!"
}
// replacen()
// 该方法多了一个字母 n,可以接受第三个参数,来指定替换的个数。同样返回一个新的字符串。
// 不会修改原始String,返回一个新的字符串
fn main() {
    let string_replace = "I like rust. Learning rust is my favorite!";
    let new_string_replacen = string_replace.replacen("rust", "RUST", 1);
    dbg!(new_string_replacen);  // I like RUST. Learning rust is my favorite!
}

// replace_range()
// 该方法仅适用于 String 类型, 接受两个参数, 第一个为被替换的范围, 第二个为替换的字符串
// 该方法直接操作原来字符串, 不返回新字符串, 所有被替换的字符串需要用 mut 关键字修饰。
fn main() {
    let mut string_replace_range = String::from("I like rust!");
    string_replace_range.replace_range(7..8, "R");
    dbg!(string_replace_range); // I like Rust!
}

// 删除
// 与字符串删除相关有四个方法, 都 仅适用 于 String 类型：

//pop()
// 直接操作 原来的字符串, 删除最后一个字符, 并返回被删除的字符。
// 尽管是直接操作原字符串,但是存在返回值,返回的是 Option 类型,如果字符串为空,则返回 None。
// pop 弹栈除了最后一个元素
fn main() {
    let mut string_pop = String::from("rust pop 中文!");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);
}

// p1 = Some(
//     '!',
// )
// p2 = Some(
//    ' 文 ',
// )
// string_pop = "rust pop 中 "

//remove()
//直接操作 原来的字符串,接受一个参数,指定删除位置,并返回被删除的字符.
// 该方法是按照字节来处理字符串的,所以位置参数要落在合法的字符边界
fn main() {
    let mut string_remove = String::from(" 测试 remove 方法 ");
    println!(
        "string_remove 占 {} 个字节 ",
        std::mem::size_of_val(string_remove.as_str())
    );
    // 删除第一个汉字
    string_remove.remove(0);
    // 下面代码会发生错误
    // string_remove.remove(1);
    // 直接删除第二个汉字
    // string_remove.remove(3);
    dbg!(string_remove);  // string_remove = " 试 remove 方法 "
}

// truncate()
// 直接操作 原来的字符串,接受一个参数,指定删除位置,删除从指定位置开始到字符串末尾的所有字符
fn main() {
    let mut string_truncate = String::from(" 测试 truncate");
    string_truncate.truncate(3);
    dbg!(string_truncate); // string_truncate = " 测 "
}
// clear()
//直接操作 原来的字符串,删除所有字符
fn main() {
    let mut string_clear = String::from("string clear");
    string_clear.clear();
    dbg!(string_clear);  // string_clear = ""
}

// + += 链接 会发生所有权转移, 第一个参数不能是引用,第二个参数为引用

fn main() {
    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    // &string_rust 会自动解引用为 &str
    let result = string_append + &string_rust;
    // string_append 所有权被转移,无法再次调用
    let mut result = result + "!";
    result += "!!!"; // 等价于 result = result + "!!!";

    println!(" 连接字符串 + -> {}", result);
}

// format!()
// format!() 不会获取任何参数的所有权。
fn main() {
    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}!", s1, s2);
    println!("{} + {} = {}", s1, s2, s); // 这里可以正常使用 s1 和 s2
}


// 操作 utf-8 字符串
// 按照字符串 操作
fn main() {
    let s = String::from(" 华中农业大学 ");
    for c in s.chars() {
        println!("{}", c);
    }
}
// 按照字节 操作 bytes()
fn main() {
    let s = String::from(" 农业 ");
    for c in s.bytes() {
        println!("{}", c);
    }
}

// 元组 tuple 由多种类型组合到一起,其长度是固定的,其中元素的顺序也是固定的
fn main() {
    let tup = (500, 6.4, 1);
    // 未使用的变量 标记下划线_
    //help: if this is intentional, prefix it with an underscore: `_x` `_z`
    let (_x, y, _z) = tup;

    println!("The value of y is: {}", y);
}

// 元组元素访问
fn main() {

    let x_tuple: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x_tuple.0;

    let six_point_four = x_tuple.1;

    let one = x_tuple.2;
    println!("{},{},{}",five_hundred,six_point_four,one)
}

// 结构体 (Struct)

// 自定义结构体  类

#[derive(Debug)]  // 自动实现Debug trait,使结构体可以通过println!("{:?}", var)输出
#[allow(dead_code)]  // ‌抑制特定编译警告‌:忽略代码中未使用的变量、函数、结构体等产生的 dead_code 警告
struct Student {  // 结构体名称为 Student,拥有如下四个字段
    active: bool, // 是否在校,布尔值类型
    name: String, // 名字, `String` 类型
    age: u8, // 年龄,u8 类型
    score: i32, // 分数,i32 类型
}

//创建结构体实例
fn main() {
    let _exps = Student {
        active: true,
        name: String::from("exps"),
        age: 18,
        score: 100,
    };
    println!("{:?}",_exps)
}

// 访问结构体字段
fn main() {
    let exps = Student {
        active: true,
        name: String::from("exps"),
        age: 18,
        score: 100,
    };
    println!("exps'name: {:?}", exps.name);  // exps'name: "exps"
}

// 修改结构体字段
fn main() {
    let mut exps = Student {  // 声明结构体实例为可变的
        active: true,
        name: String::from("exps"), // 使用String 保证结构体有变量的所有权,而不是使用 &str
        age: 18,
        score: 100,
    };
    println!("exps'name: {:?}", exps.name); // exps'name: "exps"
    exps.name = String::from("exps2");
    println!("exps'name: {:?}", exps.name);  // exps'name: "exps2"
}


// 结构体更新

let exps = Student {  // 创建第一个实例
    active: true,
    name: String::from("exps"),
    age: 18,
    score: 100,
};
let exps2 = Student {  // 创建第二个实例,除了 name,其他字段都和 exps 相同
active: exps.active,
name: String::from("exps2"),
age: exps.age,
score: exps.score,
};
let exps3 = Student {  // 创建第三个实例,可以使用结构体更新语法..exps
age: 20,  // 这个实例的 age 改为 20,其余不变
..exps  // 需要在尾部使用; exps结构体的所有权转移给了 exps3;其中结构体中 {bool,int} 有copy 特性(存储在栈中,支持复制)
        // 所以 是结构体exps中 name  String 的所有权被转移了
};


// 元组结构体 (Tuple Struct)
// 定义一个结构体必须要用名称,但是其中的字段可以没有名称,这种结构体叫做元组结构体
fn main() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black-r = {:}", black.1);
}

// 单元结构体 (Unit-like Struct)
// 结构体不存储内容,只考虑添加功能

fn main() {
    struct A;
    struct B;

    impl A {
        fn foo(&self) {
            println!("A::foo");
        }
    }

    impl B {
        fn foo(&self) {
            println!("B::foo");
        }
    }

    let a = A;
    let b = B;
    a.foo();
    b.foo();
}

// 枚举 (enumeration)
#[derive(Debug)]     // 自动实现Debug trait,使结构体可以通过println!("{:?}", var)输出
#[allow(dead_code)]  // ‌抑制特定编译警告‌:忽略代码中未使用的变量、函数、结构体等产生的 dead_code 警告
enum PokerSuit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

fn main() {
    let heart = PokerSuit::Hearts;
    let spade = PokerSuit::Spades;

    print_suit(heart);
    print_suit(spade);
}

fn print_suit(suit: PokerSuit) { // 类型为枚举PokerSuit
    println!("Suit is {:?}", suit);
}

// (1)枚举之间 关联 简化

enum PokerSuit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

struct PokerCard {
    suit: PokerSuit, // 花色
    value: u8,       // 点数
}
fn main() {
    let card1 = PokerCard {
        suit: PokerSuit::Spades,
        value: 1,
    };
    let card2 = PokerCard {
        suit: PokerSuit::Hearts,
        value: 2,
    };
}

// (2)枚举之间 关联 简化
use std::fmt;
enum PokerSuit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}
enum PokerCard { // 再定义一个枚举类型
    Spades(u8),
    Hearts(u8), // 直接把信息关联到枚举成员上
    Diamonds(u8),
    Clubs(u8),
}

// 为PokerCard实现Display trait
impl fmt::Display for PokerCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PokerCard::Spades(n) => write!(f, "♠{}", n),
            PokerCard::Hearts(n) => write!(f, "♥{}", n),
            PokerCard::Diamonds(n) => write!(f, "♦{}", n),
            PokerCard::Clubs(n) => write!(f, "♣{}", n),
        }
    }
}


fn main() {
    let card1 = PokerCard::Spades(1);
    let card2 = PokerCard::Hearts(2);
    println!("card1:{}",card1);
    println!("card2:{}",card2);
}

// 枚举包含 变量类型

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let m1 = Message::Quit;
    let m2 = Message::Move{x:1,y:1};
    let m3 = Message::ChangeColor(255,255,0);
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let m1 = Message::Quit;
    let m2 = Message::Move{x:1,y:1};
    let m3 = Message::ChangeColor(255,255,0);
}
/*
Quit 成员不携带任何数据
Move 成员携带了一个结构体
Write 成员携带了一个字符串
ChangeColor 成员携带了三个整数

这也相当于定义了四种不同的结构体

struct QuitMessage; // 单元结构体
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // 元组结构体
struct ChangeColorMessage(i32, i32, i32); // 元组结构体

*/

///////////////
// 数组

// 固定长度数组 array:速度快、长度固定
// 可变长度数组 vector:可动态增长、有性能损耗

// 固定长度数组 array 三要素
// 长度固定
// 元素类型相同
// 依次线性排列

// 创建数组
let arr1 = [1, 2, 3, 4, 5]; // 基本语法
let arr2: [i32; 5] = [1, 2, 3, 4, 5]; //显式指定数组元素类型和长度
let arr3 = [0; 5]; // [0, 0, 0, 0, 0] // 语法糖：初始化一个某个值重复出现 N 次的数组:[value; N]

// 访问数组元素

fn main() {
    let a = [9, 8, 7, 6, 5];

    let first = a[0]; // 获取 a 数组第一个元素
    let second = a[1]; // 获取第二个元素
}
// 越界访问 返回 panic 报错

// 数组切片
#![allow(unused)]
fn main() {
let a: [i32; 5] = [1, 2, 3, 4, 5];

let slice: &[i32] = &a[1..3];

assert_eq!(slice, &[2, 3]);
}

// 数组切片有如下特点：

// 切片的长度可以与数组不同,并不是固定的,而是取决于你使用时指定的起始和结束位置
// 创建切片的代价非常小,因为切片只是针对底层数组的一个引用
// 切片类型 [T] 不固定,但是切片引用类型 &[T]是固定大小,Rust 喜欢这种类型



// 可变长度数组 vector
fn main() {
    // 存储 &str 类型
    let vec_str = vec!["xx"; 10];
    println!("{:?}", vec_str); // 输出 ["xx", "xx", "xx", ..., "xx"]
    
    // 如果需要 String 类型
    let vec_string = vec!["xx".to_string(); 10];
    println!("{:?}", vec_string); // 输出 ["xx", "xx", "xx", ..., "xx"]
}

// 字符串的添加数组

fn main() {
    // 创建空 vector(指定类型为 Vec<String>)
    let mut vec = Vec::<String>::new();
    
    // 方式1:添加字符串字面量(需要转换为 String)
    vec.push("hello".to_string());
    
    // 方式2:添加已存在的 String 变量(所有权转移)
    let s = String::from("world");
    vec.push(s); // 此时 s 的所有权转移到 vector
    
    // 方式3:克隆字符串保留原变量所有权
    let s2 = String::from("rust");
    vec.push(s2.clone()); // 克隆副本加入 vector
    
    // 方式4:使用 + 运算符合并字符串后添加
    let part1 = String::from("hello");
    let part2 = String::from(" world");
    vec.push(part1 + &part2);
    
    // 方式5:使用 format! 宏
    let name = "Alice";
    let age = 25;
    vec.push(format!("{} is {} years old", name, age));
    
    // 方式6:追加多个元素
    vec.extend(vec![
        "element1".to_string(),
        "element2".to_string()
    ]);
    
    // 方式7:插入到指定位置
    vec.insert(0, "first".to_string());
    
    println!("{:?}", vec);

    //for item in vec.iter() {
    for item in &vec {
    //    for item in vec { // vec所有权被转移,后面无法继续使用vec变量
            println!("{:?}", item);
        }
        println!("{:?}", vec);
}

//// 逻辑分支判断


fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}


// ELSE-IF 多重条件分支 # else if 有点像 awk
fn main() {
    let n = 6;

    if n % 4 == 0 {
        println!("number is divisible by 4");
    } else if n % 3 == 0 {
        println!("number is divisible by 3");
    } else if n % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

// for 循环

// 1.
for item in collection {
    // do something
}

// 2.

for item in IntoIterator::into_iter(collection) {
    // do something
}

// 1 和 2 两种方式等价

// 对于集合对象,一般使用引用形式,如果不是的话,所有权就会被 move 到循环体内,导致循环体外的对象无法使用。
// 1.
for item in &collection {
    // ...
  }
  // 2.
  for item in collection.iter() {
    // ...
  }
    //for item in vec.iter() {
    // for item in &vec {
    for item in vec { // vec所有权被转移,后面无法继续使用vec变量
        println!("{:?}", item);
    }
    println!("{:?}", vec);
  // 1 和 2 两种方式等价


// 不过对于实现了 拷贝 特征的对象,比如数组,可以直接使用值形式,并不会被转移所有权
  fn main() {
    let a = [1, 2, 3, 4];
    for item in a {
        println!("{}", item);
    }
    println!("{:#?}", a);
}


// 如果要在循环中修改集合中的元素,可以使用 mut 关键字
// 1.
for item in &mut collection {
    // ...
  }
  // 2.
  for item in collection.iter_mut() {
    // ...
  }
  
  // 1 和 2 两种方式等价


// 在循环中获取索引,可以使用 enumerate 方法
fn main() {
    let a = [4, 3, 2, 1];
    for (i, v) in a.iter().enumerate() {
        println!(" 第 {} 个元素是{}", i + 1, v);
    }
}


// 对于不想在循环体中使用的变量,可以用 _ 来代替,编译器会自动忽略


fn main() {
    for _ in 0..10 {
        println!("Hello, world!");
    }
}


// while 循环

fn main() {
    let mut n = 0;

    while n <= 5  {
        println!("{}!", n);
        n = n + 1;
        // break; 也可以使用
    }

    println!("LIFTOFF!!!");
}

// loop 和 break

// loop 是一个简单的无限循环,在内部实现逻辑中,可以使用 break 来跳出循环
fn main() {
    let mut counter = 0;

    let result = loop {  // loop 返回值是一个表达式
        counter += 1;

        if counter == 10 {
            break counter * 2;  // break 返回值
        }
    };

    println!("The result is {}", result);
}

// break 既可以跳出循环,也可以返回值
// loop 的返回值是一个表达式,所以可以直接赋值给一个变量



///////////////////////////// 匹配条件 分支
// match 匹配

fn main() { // 判断是不是,返回值
    match target {
        模式 1 => 表达式 1,
        模式 2 | 模式 3 => {
            语句 1;
            语句 2;
            表达式 2
        },
        _ => 表达式 3
    }
}
// target 是要进行匹配的值,模式 是要匹配的模式,表达式 是要执行的代码

fn main() {
    enum Coin { // 定义枚举类型
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    // 判断 类型
    fn value_in_cents(coin: Coin) -> u8 {
        match coin { // 判断是不是,返回值
            Coin::Penny =>  {
                println!("Lucky penny!");  // 语句
                1 // 最后一行要是表达式
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
    // 正确调用函数并打印返回值
    let result = value_in_cents(Coin::Penny);
    println!("Coin value: {}", result);


    
    // 为 Coin 实现 Display trait
    impl std::fmt::Display for Coin {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self { // 判断是不是,返回值
                Coin::Penny => write!(f, "Penny"),
                Coin::Nickel => write!(f, "Nickel"),
                Coin::Dime => write!(f, "Dime"),
                Coin::Quarter => write!(f, "Quarter"),
            }
        }
    }

    // 然后可以直接打印枚举值
    let coin = Coin::Dime;
    println!("This is a {}", coin);

}

//match 作为表达式,进行赋值
enum IpAddr {
    Ipv4,
    Ipv6
 }
 
 fn main() {
     let ip1 = IpAddr::Ipv6;
     let ip_str = match ip1 {
         IpAddr::Ipv4 => "127.0.0.1",
         _ => "::1",
     };
 
     println!("{}", ip_str);
 }


// 穷尽匹配
// match 的匹配必须要穷尽所有情况,不然不能通过编译,如下代码
// 如果对象是枚举,则需要定义枚举中全部情况
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        },
    };
}



// _ 通配符 代替未穷举的内容
// 当不想列出所有的匹配情况时,可以使用 _ 通配符,如下代码

#![allow(unused)]
fn main() {
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),//() 返回单元类型,即不做任何事
    }
}

// if let 匹配 替代match
#![allow(unused)]
fn main() {
    let v = Some(3u8);
    if let Some(3) = v { // 条件执行
        println!("three");
    }
}

// matches! 宏 匹配 返回bool true false
// match 判断bool 判断一个字符是否是字母
fn main() {
    let foo = 'f';
    println!("{}", get_foo(foo));
}

fn get_foo(c: char) -> bool {
    match c {
        'A'..='Z' => true,
        'a'..='z' => true,
        _ => false,
    }
}

// matches! 宏简化
fn main() {
    let foo = 'f';
    println!("{}", get_foo(foo));
}

fn get_foo(c: char) -> bool {
    // 闭区间包含Z
    matches!(c, 'a'..='z' | 'A'..='Z') // ..= 表示闭区间,.. 表示开区间,可以用来匹配序列中的值,当模式匹配任何在此序列内的值时,该分支会执行
}


// Option 用法
// Option 是一个枚举类型,它的定义如下

enum Option<T> {
    Some(T),
    None,
}
// 用match 判断 Option 枚举的两种类型
#![allow(unused)]
fn main() {
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // 判断值 => 返回值
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
}


// 模式是 Rust 中的特殊语法,它用来匹配类型中的结构和数据,它往往和 match 表达式联用,以实现强大的模式匹配能力。模式一般由以下内容组合而成

// 字面值
// 解构的数组、枚举、结构体或者元组
// 变量
// 通配符
// 占位符


// match 分支
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    _ => EXPRESSION,
}

//if let 分支
if let PATTERN = SOME_VALUE {
    /// do something
}

// while let 循环
fn main() {
    // Vec 是动态数组
    let mut stack = Vec::new();
    
    // 向数组尾部插入元素
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    // stack.pop 从数组尾部弹出元素
    while let Some(top) = stack.pop() { // 判断条件,stack.pop()有返回值 取Some中数值赋值 // 一旦其返回 None,while 循环停止;返回 Some() 类型则一致while
        println!("{}", top);
    }
}


//  新循环结构
// for 循环
fn main() {
    let v = vec!['a', 'b', 'c'];
    
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

// 使用 enumerate 方法产生一个迭代器,该迭代器每次迭代会返回一个 (索引,值) 形式的元组,然后用 (index,value) 来匹配


// let 语句
let (x, y, z) = (1, 2, 3);
// 也是一种模式匹配,将一个元组解构成三个变量


// 函数参数
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}

/////////// 全模式列表  模式语法
// 匹配字面值
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
}

// 匹配命名变量
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

// 单分支多模式
// 可以用 | 来匹配多个模式,代表或的关系

let x = 1;

match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
}

// 序列匹配范围
// ..= 表示闭区间,.. 表示开区间,可以用来匹配序列中的值,当模式匹配任何在此序列内的值时,该分支会执行
// 闭区间包含5
fn main() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    println!("x = {:?}", x);
}


// 解构分解值
// 也用模式来解构结构体、元组、枚举、数组和引用

// 解构结构体 (拆分)
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point {x: 0, y: 7};

    let Point {x: a, y: b} = p; // 对应赋值
    assert_eq!(0, a);
    assert_eq!(7, b);
}


struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point {x: 0, y: 7};

    match p { // 数值情况与与结构体构造情况 对应;
        Point {x, y: 0} => println!("On the x axis at {}", x),
        Point {x: 0, y} => println!("On the y axis at {}", y),// x对应数值0 对应上了,将p中的y赋值给y
        Point {x, y} => println!("On neither axis: ({}, {})", x, y),
    }
}


// 解构枚举
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move {x, y} => { // match 顺便赋值
            println!(
                "Move in the x direction {} and in the y direction {}",
                x,
                y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        }
    }
}

// 解构嵌套的结构体和枚举
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(Color),// 嵌套枚举
}
 
fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255)); // 嵌套枚举
    // 解构 匹配
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => { // 对应的结构体嵌套形式
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h,
                s,
                v
            )
        }
        _ => () // _ 省略其他情况,返回值()
    }
}


// 解构数组
// 和元组类似

// 元组解构
let tup = (500, 6.4, 1);
let (x, y, z) = tup;

// 固定长度数组
let arr: [u16; 2] = [114, 514];
let [x, y] = arr; // 结构数组/ 元组

assert_eq!(x, 114);
assert_eq!(y, 514);

// 可变长度数组
let arr: &[u16] = &[114, 514];

if let [x, ..] = arr { // 取第一个值
    assert_eq!(x, &114);
}

if let &[.., y] = arr { // 取最后一个值
    assert_eq!(y, 514);
}

let arr: &[u16] = &[]; // 建立空切片

assert!(matches!(arr, [..]));  // [..] 匹配任意长度的切片(包括空切片)
assert!(!matches!(arr, [x, ..]));// [x,..] 匹配切片中第一个一个元素(包括空切片)
/*
‌切片模式语法‌：
[x, ..]：匹配第一个元素,忽略后续。
[.., y]：匹配最后一个元素,忽略前面。
[..]：匹配任意长度切片(包括空)。
*/


// 使用下划线 _,忽略模式中的值

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn main() {
    foo(3, 4);
}

// 使用嵌套的下划线忽略部分值

let mut setting_value = Some(5);
let new_setting_value = Some(10);

match (setting_value, new_setting_value) {
    (Some(_), Some(_)) => {
        println!("Can't overwrite an existing customized value");
    }
    _ => {
        setting_value = new_setting_value;
    }
}

println!("setting is {:?}", setting_value);
// 这个例子中,第一个匹配分支,只关心 setting_value 和 new_setting_value 是否都是 Some 值,所以使用了下划线忽略了它们的值


// 使用下划线忽略未使用的变量
fn main() {
    let s = Some(String::from("Hello!"));
    // if let 匹配 Some(),是的话执行{}内容,并且_s = s // 匹配作为表达式,进行赋值
    if let Some(_s) = s {  // 这里的 _s 仍然会绑定到 s 的值,s 的值会被移动到 _s 中  // 赋值(绑定)转移
        println!("found a string");
    }
    
    println!("{:?}", s); // s 的值已经被移动到 _s 中,所以这里报错
}

// 使用.. 忽略剩余的值
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => { // 如果有第一个和最后一个元素,则模式绑定(赋值); first = 2 ,last = 32
            println!("Some numbers: {}, {}", first, last);
        },
    }
}
// Some numbers: 2, 32


// 匹配守卫,(match guards) 允许我们在匹配模式的同时,加入额外的条件判断
// 如果这个条件判断为 true,则执行匹配分支,否则继续尝试下一个匹配分支
let num = Some(4);

match num {
    Some(x) if x < 5 => println!("less than five: {}", x),// 额外条件
    Some(x) => println!("{}", x),
    None => (),
}

//当一个匹配分支中,指定了多个模式,用 | 分隔,这时,匹配守卫的优先级是独立的,比如下面这个例子



// 使用.. 忽略剩余的值
fn main() {
    let x = 4;
    let y = false;
    
    match x {
        // 优先级 (4 | 5 | 6) if y => ...
        4 | 5 | 6 if y => println!("yes"), // 同时要求 y 必须是true
        _ => println!("no"),
    }
}

// @ 绑定,(at binding) 允许我们将匹配的值绑定到一个变量,这个变量可以在匹配分支中使用
fn main() {
    enum Message {
        Hello {id: i32},
    }
    
    let msg = Message::Hello {id: 5};
    
    match msg {
        Message::Hello {id: id_variable @ 3..=7} => { // 绑定id 范围3到7 闭区间
            println!("Found an id in range: {}", id_variable)
        },
        Message::Hello {id: 10..=12} => {
            println!("Found an id in another range")
        },
        Message::Hello {id} => {
            println!("Found some other id: {}", id)
        },
    }
}


///////////////////////////////

// 定义方法
// Rust 使用 impl 关键字来定义方法,方法是在结构体或枚举上定义的函数
// impl == implementation

#![allow(unused)]
fn main() {
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    // new 是 Circle 的关联函数,因为它的第一个参数不是 self,且 new 并不是关键字
    // 这种方法往往用于初始化当前结构体的实例
    fn new(x: f64, y: f64, radius: f64) -> Circle { //没有self参数 → 称为关联函数(不是方法)
        Circle {
            x: x,//x: x 可简写为 x(两者等价)
            y: y,
            radius: radius,
        }
    }

    // Circle 的方法,&self 表示借用当前的 Circle 结构体
    fn  ,(&self) -> f64 { // 返回值float 64
        std::f64::consts::PI * (self.radius * self.radius)
    }
    }
}

// 第二个方法
#[derive(Debug)]  // 自动实现Debug trait,使结构体可以通过println!("{:?}", var)输出
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { // impl 块
    fn area(&self) -> u32 { //&self 替代 rectangle: &Rectangle // self: &Self 的简写
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {width: 30, height: 50};

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}


// self、&self、&mut self

// 带有多个参数的方法

#![allow(unused)] // 允许代码中存在未使用的变量/函数而不触发警告 //等价写法：#[allow(unused_variables, unused_mut)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {  // 关联函数
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {  // 引用,避免所有权转移;只转移读权力
        self.width > other.width && self.height > other.height // 是否面积覆盖(基于宽高都小于self的属性)
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

// 关联函数
// 定义在 impl 中且没有 self 的函数被称之为关联函数

// Rust 中有一个约定俗成的规则,使用 new 来作为构造器的名称,出于设计上的考虑,Rust 特地没有用 new 作为关键字
#![allow(unused)] // 允许代码中存在未使用的变量/函数而不触发警告 //等价写法：#[allow(unused_variables, unused_mut)]
#[derive(Debug)] // 自动实现Debug trait,使结构体可以通过println!("{:?}", var)输出
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(w: u32, h: u32) -> Rectangle {
        Rectangle {width: w, height: h}
    }
}


// 实现 Display 格式化输出
// use std::fmt;
// println!("Rectangle struct {}", a);
impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rectangle [width={}, height={}]", self.width, self.height)// 输出
    }
}

fn main() {

    let w1 = 10;
    let h1 = 10;
    let a= Rectangle::new(w1,h1); // 调用关联函数
    println!("Rectangle struct {}", a);
}

// 为枚举类型定义方法

#![allow(unused)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // 在这里定义方法体
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}


// 泛型 Generic
// 加法函数,两个参数可能是整数,也可能是浮点数,甚至是字符串,这种时候我们不希望针对每一种类型都写一个函数,这时候就可以使用泛型

// 未使用泛型
fn add_i32 (a:i32, b:i32) -> i32 {
    a + b
}
fn add_f64 (a:f64, b:f64) -> f64 {
    a + b
}

// 使用泛型
// std::ops::Add 这个特征来约束, 泛型 可相加
fn add<T: std::ops::Add<Output = T>>(a:T, b:T) -> T {
    a + b
}
fn main() {

    let w1 = 10;
    let h1 = 10;
    let a= add(w1,h1);
    println!("add: {}", a);
}

// 结构体中的泛型
struct Point<T> {
    x: T,
    y: T,
}

fn main () {
    let integer = Point {x: 5, y: 10};
    let float = Point {x: 1.0, y: 4.0};
}
// 但是需要注意的是,x 和 y 的类型必须是一致的,否则会报错,如果想要 x 和 y 的类型可以不一致,那么就要再声明一个泛型
// U
struct Point<T,U> {
    x: T,
    y: U,
}
fn main () {
    let p = Point {x: 1, y :1.1};
}

// 枚举中的泛型

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

///////////////////
// 特征 Trait
// 一组可以被共享的行为,只要实现了特征,就可以使用这些行为

//Rust 中特征 Trait 的概念和其他语言中的接口很相似,它定义了一些方法,如果一个类型实现了某个特征,那么它就可以调用这些方法


// 实现特征
//// 定义 标注返回值,指定了关联函数的基本特征

//// summarize(&self) -> String 是一个方法签名,表示所有实现 Summary 的类型必须提供这个方法

// 没有自定义实现
// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// 有默认的自定义实现
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("Read more...")
    }
}
// 结构体
pub struct News {
    pub title: String, // 标题
    pub author: String, // 作者
    pub content: String, // 内容
}
// 为 News 和 Weibo 实现 Summary Trait,定义各自的摘要逻辑
// 多态性‌：不同类型的对象(News 和 Weibo)可以共享相同的行为(summarize 方法),但具体实现不同
impl Summary for News {
    // fn summarize(&self) -> String {
    //     format!(" 文章 {}, 作者是 {}", self.title, self.author)
    // }

    // 注释内容,使用 Summary trait 默认的summarize实现
}

pub struct Weibo {
    pub username: String,
    pub content: String
}
// 为 News 和 Weibo 实现 Summary Trait,定义各自的摘要逻辑
// 多态性‌：不同类型的对象(News 和 Weibo)可以共享相同的行为(summarize 方法),但具体实现不同
impl Summary for Weibo {
    // 不使用 Summary trait 默认的summarize实现
    // 自己自定义写summarize 的实现
    fn summarize(&self) -> String {
        format!("{} 发表了微博 {}", self.username, self.content)
    }
}
fn main() {
    let news = News{title: "title".to_string(),author: "lyz".to_string(), content: "Rust 棒极了!".to_string()};
    let weibo = Weibo{username: "lyz".to_string(),content: " 好像Tweet 没微博 好用 ".to_string()};

    println!("{}",news.summarize());
    println!("{}",weibo.summarize());
}



// 如果为类型 A 实现特征 T ，那么 A 和 T 至少一个需要在当前作用域中定义
// 就是写包 所必须的特征


// 默认实现
// 特征中的方法可以有默认实现
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("Read more...")
    }
}

//
impl Summary for News {}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{} 发表了微博 {}", self.username, self.content)
    }
}

// 特征作为函数参数
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// 这里的 item 参数可以是任意实现了 Summary 特征的类型,比如 News 或者 Weibo

// 特征约束 (Trait Bound) #指定了T 泛型 必须具有具有Summary特征
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}
//  两种写法有不同特征
pub fn notify_1(item1: &impl Summary, item2: &impl Summary) {}  // 两个参数可以是不同类型
pub fn notify_2<T: Summary>(item1: T, item2: T) {} // 两个参数必须是同一种类型,并且实现了Summary 特征(类关联函数)

// 多重约束 ,需要实现了 Summary 和 Display 两种特征
pub fn notify_1(item: impl Summary + Display) {}
pub fn notify_2<T: Summary + Display>(item: T) {}

// where 从句约束
// 当约束的特征太多的时候,使用where
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}
// where 从句
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{}

// 使用特征约束有条件地实现方法和特征

fn main() {
    use std::fmt::Display;
    
    struct Pair<T> {
        x: T,
        y: T,
    }
    
    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self {
                x,
                y,
            }
        }
    }
    
    impl<T: Display + PartialOrd> Pair<T> { // T: Display + PartialOrd 限制了T 的特性
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
}

// 函数返回值类型中使用特征约束
fn returns_summarizable() -> impl Summary {
    News {
        headline: String::from("headline"),
        location: String::from("location"),
    }
}
// 返回值类型是 impl Summary,表示返回值类型必须实现 Summary 特征
// 自身预期返回值的限制
// 复杂情况不行

// 通过 drive 宏派生特征
// Rust 提供了 derive 宏,可以自动实现一些特征,比如 Debug、Clone 等,被标记的对象会自动实现这些特征,继承相应的功能
// 比如 Debug 特征，标记后，可以使用 {:?} 打印对象
#![allow(unused)] // 允许代码中存在未使用的变量/函数而不触发警告 //等价写法：#[allow(unused_variables, unused_mut)]
#[derive(Debug)] // 自动实现Debug trait,使结构体可以通过println!("{:?}", var)输出s
