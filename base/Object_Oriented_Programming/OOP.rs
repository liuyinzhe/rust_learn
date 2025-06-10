
/*
    Rust 的面向对象编程特性
    Object-Oriented Programming Features of Rust

    对象
    
    继承

    封装
    隐藏实现细节的封装
    对象的实现细节不能被使用该对象的代码访问

    pub 关键字来决定代码中那些模块、类型、函数和方法应该是公共的
    默认情况下其他所有内容都是私有的

    继承
    继承作为类型系统和代码共享
    继承是一种机制,通过它一个对象可以继承另一个对象定义中元素,从而获得父对象的数据和行为，而无需再次定义他们

    在不适用宏的情况下，没有办法顶一个继承父结构体的字段和方式实现的结构体

    多态性
    可以处理多种类型数据的代码
    Rust 则使用泛型来抽象不同可能的理性,并使用特征边界来强制这些类型必须提供什么。这有时被称为有界参数多态性
    Rust 采用了不同方法,使用特征对象(trait object) 而不是继承


*/

/*
    Trait Object 特征对象
    
    为通用行为定义Trait
    Trait Object
    
    Rust 提供了泛型来支持抽象编程,但泛型要求类型在编译器已知。

    Trait 对象(Trait Object) 是另外一种抽象方式:在运行时支持不同类型的值,前提是他们实现了某个 trait

    类似于OOP中的"接口+多态"或"鸭子类型"


    我们通过引用类型(如&)或者智能指针(如Box<T>), 加上dyn 关键词，再指定trait 名称的方式来创建trait对象

    不能向trait 对象中添加数据字段。 trait 对象的用途较为特殊，他们主要用于在通用行为上实现抽象

*/ 


// lib

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>, //components 就是 trait object
}

// 为 Screen 实现方法
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label:String,
}

// 为 Button 实现 Draw trait
impl Draw for Button {
    fn draw(&self) { // 不添加内容就表示与 原先Draw trait 函数内容一致；否则替换功能
        // code to actually draw a button
    }
}

// main bin // cargo build gui

use gui::Draw

struct SelectBox {
    width: u32,
    height: u32,
    options:Vec<String>,
}

// 为 SelectBox 实现 Draw trait
impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
        println!("Drawing SelectBox");
    }
}

fn main() {
    let screen = Screen {
        components = vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybve"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label:String::from("OK"),
            }),
        ],
    };

    screen.run()
}
