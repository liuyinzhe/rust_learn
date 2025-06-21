/*
    高级Trait

    在trait 定义中使用关联类型指定占位符类型

    关联类型

    关联类型将一个 "类型占位符" 绑定到trait 中
    允许 trait 的方法使用这些 占位类型
    实现该trait时, 具体实现者提供实际的类型

*/

// 示例
// 关联类型  vs 泛型参数

//关联类型
pub trait Iterator {
    type Item; // 关联类型，表示同一种类型
    
    fn next(&mut self) -> Option<self::Item>;
}

impl Iterator for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        // --ship
    }
}
// 泛型参数
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}

/*
    关联类型  vs 泛型参数

    如果使用泛型
        可以对同一类型多次实现iterator(T 不同)
        每次调用 next 都需要明确类型
    如果是关联类型
        限定trait 只能对一个类型实现一次
        无需在使用中注明类型
        trait 定义更清晰、约定更严格
        称为trait接口的一部分
*/

// 默认泛型类型参数与运行符重载

/*
    默认泛型类型参数

    当使用泛型类型参数时, 可以为其指定一个默认的具体类型
    当默认类型适用时, 实现该trait的人就无需额外指定类型
    语法:<占位类型=默认类型>。

    运算符重载概览

    Rust 不允许创建自定义运算符,也不能重载任意运算符
    但可以通过实现std::ops中定义的trait来重载已有的运算符

*/

// trait Add<Rhs=Self> { // 类型默认就是 Self
//     type Output;

//     fn add(self, rhs:Rhs) -> Self::Output;
// }

//  使用默认类型
use std::ops::Add;

#[derive(Debug,Copy,Clone,PartialEq)]

struct Point {
    x:i32,
    y:i32,
}

// 为Point 结构体 实现 Add trait  //use std::ops::Add;
impl Add for Point {
    type Output = Point; // 关联类型 //type 占位类型=默认类型

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }

    }
}

fn main() {
    assert_eq!(
        Point {x:1, y:0} + Point {x:2,y:3},
        Point {x:3,y:3}
    );
}

// 不适用默认类型
use std::ops:Add;

struct Millimeters(u32); // 毫米

struct Meters(u32);// 米

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other:Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000)) // 毫米 + 米,返回毫米
    }
}

/*
    默认类型参数的两大用途

    扩展 trait 而不破坏现有代码
    允许特定情况下的自定义


    完全限定语法(Fully Qualified Syntax)
    用于消除歧义

    Rust 允许多个trait拥有相同的方法名
    与可以在同一类型上实现多个带有相同方法名的trait
    类型本身也可以定义相同名字的方法
*/ 

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("UP!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn main() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly(); // 自己的方法

}

// 无self 关联类型

// trait 定义
trait Animal {
    fn baby_name() -> String; // 定义实现函数的返回类型
}

struct Dog;

// 自定义 Dog 自己的方法
impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called a {}", Dog::baby_name());// 自身的方法
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());//指定类型使用类型方法// Animal trait 方法
}

// 完全限定语法
<Type as Trait>:: function(receiver_if_method, next_arg, ...);

/*
    Supertrait

    在Rust中, 如果一个Trait 依赖于另一个Trait的功能, 我们可以将这个被以来的Trait作为Supertrait引入
        这让我们可以在Trait内部直接使用另一个Trait提供的功能
*/

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string(); //to_strint trait 来自 fmt::Display
        let len = output.len();
        println!("{}","*".repeat(len + 4));
        println!("*{}*"," ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*"," ".repeat(len + 2));
        println!("{}","*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}
// 为Point 实现 Display trait
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})",self.x, self.y)
    }
}

// 为 Point 实现 OutlinePrint
// OutlinePrint 需要 实现了 Display trait 的Point
impl OutlinePrint for Point {}

fn main() {
    let p = Point { x: 1, y: 2};
    p.outline_print();
}


/*
    使用 Newtype 模式在外部类型上实现外部 trait

    孤儿规则: 只有当trait 或类型时我们的crate时(本地),才允许我们在类型上实现 trait
    可以使用newtype 模式绕过这个限制
    该模式涉及到tuple struct 中创建一个新类型
        该 tuple struct 将有一个字段,时我们想要实现的trait的类型的轻量级包装器
        包装器类型就是属于我们crate的(本地的), 可以在包装器上实现trait
    使用这种模式没有运行时性能损失,包装器类型在编译时被忽略。

*/

use std::fmt;

struct Wrapper(Vec<String>); // 通过结构体 Vec 包装外部类型

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(","))// join是 String的 trait 方法
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"),String::from("world")]);
    println!("w = {w}");
}
