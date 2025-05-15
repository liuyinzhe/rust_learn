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
