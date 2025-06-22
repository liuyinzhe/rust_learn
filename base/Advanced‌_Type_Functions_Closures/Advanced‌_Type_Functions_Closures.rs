/*
    高级类型、函数和闭包
    Advanced Types,Functions & Closures

    高级类型
    使用Newtype 模式实现类型安全和抽象

    增强类型安全
    提供抽象
    避免不同单位的数值混淆

    // Newtype 模式
    // u32 使用struct 分为不同类型
    struct Millimeters(u32);
    struct Meters(u32);
    使用Millimeters 类型作为函数参数时, 不能误传Meters类型或u32类型

    Newtype 模式隐藏实现细节
    提供仅包含公共方法的API
    隐藏底层解构(如 HashMap)
    struct People(HashMap<i32,String>);
    外部代码只能通过我们提供的方法与People交互,无法直接访问HashMap

    
    
    使用类型别名创建类型同义词
    Creating Type Synonyms with Type Aliases

    使用 type 关键字定义类型别名
    type Kilometers = i32;
    Kilometers 与 i32 实际是相同的类型
    无法获得Newtype 模式提供的类型安全性
    类型同义词的主要用途是减少重复

    Box<dyn fn() + Send + 'static>
*/

fn main() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers =5;

    println!("x + y = {}", x+y)
}

fn main() {
    let Thunk = Box::new(|| println!(hi));
}

// 类型别名
type Thunk = Box<dyn fn() + Send + 'static>;

fn takes_long_type(Thunk) {
    // --snip--
}

fn returns_long_type() -> Thunk {
    // --snip--
}


// 例子
fn main() {

}
use std::fmt;
use std::io::Error;
// 类型别名,避免 复杂类型描述 简化统一;方便编写
type Result<T> = std::result::Result<T, std::io::Error>;
pub trait Write {
    // fn Write(&mut self, buf: &[u32]) -> Result<usize,Error>;
    // fn flush(&mut self) -> Result<(),Error>;

    // fn write_all(&mut self, buf: &[8]) -> Result<(),Error>;
    // fn flush_all(&mut self, fmt: fmt::Arguments) -> Result<(),Error>;

    fn Write(&mut self, buf: &[u32]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[8]) -> Result<()>;
    fn flush_all(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

/*
    永不返回的类型
    The Never Type that Never Rturns
    书写: !
    类型理论中称为空类型(Empty Type)
    表示"永不返回"的值

    fn bar() -> ! {
        // --snip--
    }
*/

let guess: u32 = match guess.trim().parse() { // match 类型必须一致
    Ok(num) => num,
    Err(_) => continue,  // continue 返回的 ! Never Type
};

/*
    动态大小类型和Sized trait
    Dynamically Sized Types and the Sized Trait

    大小在编译时未知
    例如: str 类型(不是&str) 为动态大小类型


*/

fn main() {
    // str 动态大小类型, 无法直接调用
    // let s1: str = "Hello there!";
    // let s2: str = "How's it going?";

    // str 动态大小类型,可以放在某种指针后使用
    let s1: &str = "Hello there!";
    let s2: &str = "How's it going?";
}

// Sized trait 

// 表面写法
fn generic<T> (t:T) {
    // --snip--
}
// 默认构成
fn generic<T:Sized> (t:T) {
    // --snip--
}
// 设定为动态大小类型 ?Sized
fn generic<T:?Sized> (t:&T) { // 动态大小类型 必须放在指针后面使用 &T
    // --snip--
}

/*
    高级函数和闭包

    Function Pointer 函数指针,将函数作为参数

    类型: fn(小写f), 区别于闭包 trait Fn
    语法: fn(参数类型) -> 返回类型
    fn 是类型
    函数指针实现了所有三种闭包特性(Fn,FnMut,FnOnce)
    可以在需要闭包的地方使用函数指针

*/

fn add_one(x: i32) -> i32 {
    x + 1
}

// fn(i32) -> i32 第一个参数
fn do_twice(f: fn(i32) -> i32, args: i32) -> i32{
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one,5);
    println!("The answer is: {answer}");
}

/*
    返回闭包

    闭包都是由特性表示的,不能直接返回
    每个闭包都有自己独特的类型
    
    解决方案1: 使用impl Trait
    fn returns_closure() -> impl Fn(i32) ->i32 { // 闭包 trait Fn
        |x| x + 1
    }

    解决方案2: 使用Trait Object
*/

fn main() {
    // 基于Trait Object,获得两个不同的闭包类型
    /*
        [
        Box<ClosureTypeA> as dyn Fn,
        Box<ClosureTypeB> as dyn Fn
        ]
    */
    let handlers = vec![returns_closure(),returns_initialized_closure(123)];

    for handler in handlers {
        let output = handler(5);
        println!("output");
    }
}

// 使用智能指针Box 包裹 Trait 作为Trait Object返回
// dyn Fn：动态分发的函数trait
// Box：堆分配保证固定大小
// 允许存储不同类型但相同trait的闭包
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    // 简单闭包
    Box::new(|x| x + 1)
}

fn returns_initialized_closure(init:i32) -> Box<dyn Fn(i32) -> i32> {
    // 捕获外部变量的闭包 // move 强制闭包获取init的所有权
    Box::new(move |x| x + init) // move关键字捕获所有权
}
