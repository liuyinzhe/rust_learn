
/*
    Unsafe Rust 

    Rust 默认保证内存安全
    Unsafe Rust 不强制这些保证
    用于更底层的系统编程或绕过编译器限制

   代码安全，但是编译器不理解
   硬件本身不安全
   支持系统级编程 


   Unsafe Rust 的五个超能力

   解引用 裸/原始指针
   调用不安全函数或方法
   访问或修改可变静态变量
   实现不安全的trait
   访问 union 的字段


   Unsafe != 不安全
   
   borrow checker 仍然工作 // Borrow checker是rust语言中保证引用(reference)安全性的重要机制。通过borrow checker机制,rust能防止虚悬引用(dangling reference)的出现。
   只是某些检查不再进行(超能力)
   代码是否安全由程序员来保证
   建议将unsafe块限制得尽可能小


   封装 unsafe , 构建安全AIP

   使用 safe API 包裹 unsafe 代码
   避免 unsafe 泄漏到用户层
   标准库就是这样做的


   解引用原始指针

   Rust 的普通引用总是有效,这是由编译器保证的
   但是 Unsafe Rust 中,出现了两种新类型的指针，称为原始指针(raw pointer):
        *coust T: 不可变原始指针
        *mut T : 可变原始指针
        "不可变" 是指: 该指针在被解引用之后，不能直接被赋值。
        注意 星号*式类型表达式的一部分,不是解引用操作符


    原始指针 vs 普通引用 & 智能指针

    原始指针的特点:
        可以忽略借用规则(同时存在多个可变或不可变指针)
        不保证指向有效内存
        可以为null
        不会自动清理(没有Drop)
    
    使用原始指针是放弃安全性以换取性能, 或是为了与像C语言或底层硬件交互。

*/ 

fn main() {
    let mut num:i32 = 5;
    // 可以在正常区块中建立原始指针,无法在unsafe块外 解引用
    // 原始指针,可以同时设置可变与不可变指针
    let r1:*const i32 = &raw const num; 
    let r2:*mut i32 = &raw mut num;


    // let address: usize = 0x012345uszie;
    // let r:*const i32 = address as *const i32;

    unsafe {
        println!("r1 is: {}",*r1);
        println!("r2 is: {}",*r2);
    }

}

/*
    调用不安全的函数/方法

    Unsafe 函数看起来与普通函数完全相同
    区别在于定义前面添加了 unsafe 关键词
    这表明函数有特定要求,Rust 无法自动保证这些要求被满足
    调用unsafe函数时,我们表示已阅读其文档并承担责任

    
*/

fn main() {
    unsafe {
        dangerous();
    }
}
unsafe fn dangerous() {}

/*
    创建 unsafe 代码的安全抽象

    包含 unsafe 代码的函数不一定要标记为 unsafe
    用安全函数封装unsafe 代码是常见的抽象模式
*/


fn main() {
    let mut v:vec<i32> = vec![1,2,3,4,5,6];

    let r:&mut[i32] = &mut v[..];

    // let (a:&mut[i32],b:&mut[i32]) = r.split_at_mut(mid:3); // 切片自带函数
    let (a:&mut [i32],b:&mut [i32]) = split_at_mut(r,mid:3);
    assert_eq!(a,&mut [1,2,3]);
    assert_eq!(b,&mut [4,5,6]);

}
use std::slice;

fn split_at_mut(values: &mut [i32],mid:usize) -> (&mut [i32],&mut [i32]) {
    let len:usize = values.len();
    let ptr:*mut i32 = values.as_mut_ptr(); // 访问切片的可变原始指针

    assert_eq!(mid<=len);// 不小于总长度,则造成panic
    unsafe {
        (   // from_raw_parts_mut 不安全
            slice::from_raw_parts_mut(ptr,mid), // 参数:原始指针,长度
            slice::from_raw_parts_mut(ptr,add(mid),len:len-mid),// add 坐标移动
        )
        //(&mut values[..mid],&mut values[mid..]) // 返回两个切片的元组 // 对切片进行了两次借用
    }
    
}


/*
    使用extern 调用外部代码

    外部函数接口(FFI) Foreign Function Interface
    Rust 代码有时需要与其他语语言编写的代码交互

    extrn 关键字用于创建和使用外部函数接口(FFI)
    FFI 使用一种编程语言能够定义函数并允许另外一种(外部)编程语言调用这些函数
*/

unsafe extrn "C" { // ABI
    // abs 函数签名
    safe fn abs(input: i32) -> i32; // 如果外部函数 已知安全并且 标记了safe,则可以跳出unsafe代码块
}

fn main() {
    println!("Absolute value of -3 according to C: {} ", abs(input:-3));
    // unsafe{
    //     println!("Absolute value of -3 according to C: {} ", abs(input:-3));
    // }
}


/*
    访问或修改可变静态变量

    Rust支持全局变量,但可能与Rust 的所有权规则产生 问题
    如果两个线程访问同一个可变全局变量，可能导致数据竞争
    Rust中的全局变量被称为*静态(static)*变量
*/

static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("name is: {HELLO_WORLD}");
}

/*
    静态变量的特性

    静态变量只能存储具有'static声明周期的引用
    Rust编译器可以推断生命周期,不需要显示注释
    静态变量在内存中有固定的地址
        使用该值将始终访问相同的数据
    常量可能在使用复制其数据
    静态变量可以是可变量(与常量不同)***
*/

static mut COUNTER: u32 = 0; // 可变静态变量 被人为不安全

unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    unsafe {
        add_to_count(inc:3);
        println!("COUNTER: {}",*(&raw const COUNTER)); // *(原始指针) 解引用
    }
}

/*
    实现unsafe trait

    可以使用unsafe来实现不安全 trait
    当trait中至少有一个方法具有编译器无法验证的不变量时,该trait就是不安全的
    使用unsafe 关键词声明 trait为不安全的

*/

unsafe trait Foo {
    // methods go here
}
 
unsafe impl Foo {
    // method implementations here
}

fn main() {}

/*
    访问 Union 的字段

    Union 类似于 struct
        在特定实例中,同一时间只能使用一个声明的字段

    主要用途: 与C代码中的Union 交互
    访问Union的字段时unsafe的
        基因 Rust 无法保证当前存储在Union实例中的数据类型
    
    有关Union的更多信息,可参考Rust参考手册
*/

/*
    什么时候使用unsafe代码

    使用unsafe 并不是错误
        但它更加棘手,因为编辑器无法帮助保证内存安全
    何时使用unsafe
        在有明确理由时,可以使用不安全代码
    如何确保代码正确性
        当编写不安全代码时,可以使用Miri来确保代码符合Rust的规则

*/
