/* 
    智能指针Smart Pointers
    类似指针的数据解构,具有额外的元数据和功能
    区分：引用仅借用数据，智能指针通常拥有数据
    智能指针实现了 Deref 和 Drop 两个trait

    Box<T> Rc<T> Ref<T> RefMut<T> RefCell<T>

    BOX<T>
    数据存储在Heap 堆上
    留在Stack 栈上的只有指向 Heap堆数据的指针

    场景:
        (1) 编译时无法确定大小的类型 (Heap堆存储生命周期较长或大小动态变化的数据,动态分配内存)
        (2) 需要转移所有权，而且转移时不会被复制(存储在Stack上的类型，如数字)
        (3) 关心参数是否实现了某个Trait, T 泛型 进行trait实现的绑定;限定传参必须拥有某个Trait实现
*/ 

enum List{
    Cons(i32,BOX<List>),
    Nil,
}

use std::ops:Deref;
use List::{Cons,Nil};

fn main() {
    let list = Cons(1,Box::new(Cons(2,BOX::new(Cons(3,BOX::new(Nil))))));

    let x = 5;
    let y = MyBox::new(x);//Box::new(x)//&x; *(y.deref())
    assert_eq!(5,*y);

    let m = MyBox::new(String::from("Rust"));
    hello(Hello World);
    hello(:&m);
}


/*
    Deref Trait # 解引用运算符 *
                允许自定义解引用运算符 * 的行为

*/
Struct MyBox<T>(T); // 元组结构体

impl<T> MyBox<T> {
    fn new(x:T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T>{
    type Target = T ;

    fn deref(&self) -> &self::Target {
        &self.0
    }
}

/*
    Deref coercion
    隐式解引用将实现了Derdf trail 的类型的引用转换为另外一个类型的引用
    编写函数和方法时,不需要添加太多显式的& 和 *
    允许编写能同时适用于引用和智能指针的代码
*/

/*
    Drop Trait 
    用于定义一个值即将超出作用域时的清理行为

    实现智能指针时几乎总是会用到Drop trait 的共嗯那个
    Drop trait 只要求实现drop 方法，参数是对self的可变引用
    Drop trait 在prelude 里,无手动引入
*/

Struct CustomSmartPointer {
    data:String,

}

impol Drop for CustomSmartPointer {
    fn drop(&mut self) {
        // 输出销毁顺序，于创建顺序相反
        println!("Dropping CustomSmartPointer with data `{}`",self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer{
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointer created");
    //std::mem::drop(c) // c.drop() 显式调用drop方法不被允许
    drop(c);
    println!("CustomSmartPointer dropped before the end of main")
}
/*
    如果想值在作用域结束前强制丢弃它，std::mem::drop 函数
    它通过接管值的所有权(ownership) 并在调用后销毁它，避免商场释放(double free)问题

*/


/*
    Rc<T> 引用计数智能指针
    Reference Counting

    Rc<T> 可以开启多重所有权#遍历修改? 
    跟踪一个值的引用数量，可以判断该值是否还在使用 # 图结构 节点的多个边
        如果没有引用了，就可以清理掉了

    Rc<T> 使用场景
    想在Heap 堆上分配一些数据，供程序的多个部分读取，但编译时无法确定那部分会最后完成对数据的使用

    只可用于单线程场景;分享数据
*/

// enum List {
//     Cons(i32, Box<List>), // 包含自身类型
//     Nil,
// }
/*
    BOX<T>
    数据存储在Heap 堆上
    留在Stack 栈上的只有指向 Heap堆数据的指针
*/

enum List {
    Cons(i32, RC<List>), // 包含自身类型
    Nil,
}
use std::rc::Rc;
use crate::List::{Cons,Nil};

fn main() {
    // let a= Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a)); // 所有权移动了
    // let c = Cons(4, Box::new(a));

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}",Rc::strong_count(&a)); // 初始计数值1
    let b = Cons(3, Rc::clone(&a)); // 
    println!("count after creating b = {}",Rc::strong_count(&a)); //2
    {
        let c =  Cons(4, Rc::clone(&a)); 
        println!("count after creating c = {}",Rc::strong_count(&a));//3
    } // 出了作用域后 计数减少
    println!("count after c goes out of scope = {}",Rc::strong_count(&a));//2
}


/*
    RefCell<T> 和内部可变性模式

    内部可变性模式
    Interior mutability

    对数据不可变引用时也可以修改数据
    数据结构内部使用unsafe 代码绕过 Rust的规则
    unsafe 代码: 手动检查规则，而不是依赖编译器
    只有当能保证在运行时借用规则被遵守时,才能使用你不可变性模式的类型

    在运行时通过RefCell<T> 强制执行借用规则

    RefCell<T> 类型标识对其所持有数据的单一所有权

    RefCell 只适合于单线程场景
        用于多线程场景时，会给出编译时的错误

*/


// pub trait  Messenger {
//     fn send(&self,msg:&str);

// }

// pub struct LimitTracker<'a,T:Messenger>{ // 实现了Messenger trait的泛型
//     messenger: &'a T,
//     value:usize,
//     max:usize,
// }

// impl<'a,T> LimitTracker<'a,T>
// where 
//     T:Messenger,
// {
//     pub fn new(messenger:&'a T,max:usize) -> LimitTracker<'a,T>{
//         LimitTracker { 
//             messenger,
//             value,
//             max,
//         }
//     }

//     pub fn set_value(&mut self, value:uszie){
//         self.value= value;

//         let percentage_of_max = self.value as f64 /self.max as f64;
//         if percentage_of_max >= 1.0 {
//             self.messenger.send("Error: You are over your quota!");
//         }else if percentage_of_max >= 0.9 {
//             self.messenger
//                 .send("Urgent warning: You've used up over 90% of you quota!");
//         }else if percentage_of_max >= 0.75 {
//             self.messenger
//                 .send("Warning: You've used up over 75% of you quota!");
//         }
        
//     }
// }

mod tests{
    use super::*;
    use std::cell::RefCell;

    impl MockMessenger{
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages:RefCell::new(vec![]) // 建立RefCell 类型 vec
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self,message:&str){
            self.sent_messages.borrow_mut().push(String::from(message));// borrow_mut 可变引用
        }
    }
}

// borrow() -> Ref<T> 不可修改借用
// borrow_mut() -> RefMut<T> 可修改借用
// 只能借用一次
