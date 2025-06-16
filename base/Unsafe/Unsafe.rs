
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
