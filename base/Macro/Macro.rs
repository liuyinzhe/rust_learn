/*
    Macro 宏

    在Rust中是一个家族，宏主要分为两大类
        声明式宏(Declaratie Macros), 也就是通过 macro_rules!定义的宏。
        过程宏(Procedural Macros), 它又可以分为三种不同的形式:
            自定义的#[derive]宏,它们用于结构体(struct)或枚举(enum)上，配合derive属性,自动为类型生成一些代码。
            类似属性的宏(Attribute-like macros), 它们可以用作任意代码项上的自定义属性。
            类似函数的宏(Function-like macros), 它们看起来像函数调用,但实际上操作的是传入的代码片段(tokens)。

    宏与函数的区别

    宏是一种用来编写代码的代码,也就是所谓的元编程
        这些宏最终会扩展成比你手写的更多的代码
    宏拥有一些函数无法做到的能力
        但宏可以接收不定数量的参数
        宏在编译器解释代码意义之前就会被展开
    定义一个宏往往比定义一个函数要复杂得多
    宏必须在使用之前定义号或导入作用域

    声明式宏(Declarative Macros)
    
    声明式宏让你可以像使用 match表达式一样,匹配代码解构并生成新的代码
    宏匹配的不是运行时的值,而是 Rust源代码的解构本身
        匹配成功后,对应的代码就会被替换到宏调用的位置
        这一些都发生在编译期间
*/

/*
    宏的定义方式

#[macro_export] 表示这个宏在当前 crate被引用作用域时也能被访问到
macro_rules! 是声明宏的关键字,后面跟着宏的名字(这里是vec,不带感叹号！
大括号{}包裹的就是宏的模式匹配和替换规则
*/
// vec! 宏 快速创建Vec
let v: Vec<u32> = vec![1,2,3];

#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}


/*
    ($($x:expr),*) => {...}
    $ 表示声明一个变量,类似shell中$取值符
    $(...)是 Rust 宏中表示重复模式的语法
    $x:expr 匹配任何Rust表达式,并将其命名为$x vec![1,2,3]
    ,表示每个表达式之间必须有逗号分隔
    *表示这个模式可以重复 零次或多次
*/

// 代码展开
let mut temp_vec = Vec::new();
temp_vec.push(1);
temp_vec.push(2);
temp_vec.push(3);


/*
    过程宏: 通过属性生成代码

    过程宏更像是函数, 它们接收一些代码作为输入,对其进行处理, 然后生成一些代码处理输出,
    而不是通过模式匹配和替换代码的方式来运作

    Rust中有三种主要的过程宏形式:
        自定义派生(custom derive)
        类似属性的宏(attribute-like)
        类似函数的宏(function-like)
    要定义过程宏,我们必须将它们写在一个单独的crate中,而且这个crate还必须具有特殊的crate类型
        这是一种目前还存在的技术限制,未来可能会被移除
*/

// 过程宏 例子
use proc_macro;
#[same_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {}

/*
    定义过程宏的函数接受一个TokenStream 类型的参数,返回一个同样类型的结果
    TokenStream是由 Rust自带的 proc_macro crate 提供的,表示一串令牌的序列,
    换句话说:
        输入的TokenStream 是宏要处理的源代码
        输出的 TokenStream 是宏生成的新代码
    在函数的上方,还需要添加一个属性(attribute), 用来表明这是哪一种过程宏
    一个crate中可以定义多种类型的过程宏
*/

/*
    如何编写一个自定义的derive宏
    Custom Derive Macro
    
    我们将创建一个名为hello_macro的crate
        它定义了一个trait: HelloMacro
        该trait有一个关联函数: hello_macro()
    我们希望用户只需在他们的类型上添加#[derive(HelloMacro)],就能自动获得一个默认实现
        这个默认实现会打印: Hello, Macro! My name is TypeName!

*/

// lib.rs
pub trait HelloMacro {
    fn hello_macro();
}

// hello_macro
// hello_macro_derive // 创建该名字的crate
// Cargo.toml 内容
[lib]
proc-macro = true // Rust 自带的, 编译器的API, 读取rust代码

[dependencies]
syn = "2.0" // 代码转换为抽象语法树(AST)
quote = "1.0" // AST 转化为 代码
// lib.rs
use proc_macro::TokenStream;
#[proc_macro_derive(HelloMacro)] // 对应指定 #[derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can maniipulate
    let ast = syn::parse(input).unwrap(); // 负责解析转化语法树
    
    // Build the trait implementation
    impl_hello_macro(&ast) 
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen_tk = quote! { // 定义返回的Rust代码 // 模板机制 #name 对应name变量
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name)); // stringify 将表达式(如 1+2) 转为为字符串
            }
        }
    };
    gen_tk.into(); // 转为 TokenStream 类型
}

// main.rs
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

// impl HelloMacro for Pancakes {
//     fn hello_macro() {
//         println!(Hello, Macro! My name is Pancakes!);
//     }
// }

fn main() {
    Pancakes::hello_macro();
}

/*
    属性宏 Attribute-like Macros

    属性宏和自定义derive宏(custom derive macros) 类似
    不同的是, 属性宏允许创建新的属性(attribute),这使得它更加灵活

    #[route(GET),"/"]
    fn index() {}

    #[proc_macro_attribute]
    pub fn route(attr:TokenStream, item: TokenStream) -> TokenStream {}

    属性宏和自定义 derive 宏的核心机制一样: 你需要创建一个专门的宏
    crate,然后写一个函数,接收 TokenStream, 生成你希望的代码


    函数宏 Function-like Macros
    函数宏看起来像是一个函数调用
        let sql = sql!(SELECT * FROM posts WHERE id=1);
    #[proc_macro]
    pub fn sql(input: TokenStream) -> TokenStream {}

    跟derive 宏很像, 函数宏也是接收一个TokenStream,生成代码返回
    跟macro_rules!宏不同的是: macro_rules!使用的是匹配规则语法;
    而函数宏使用的是Rust代码来处理TokenStream, 因此处理逻辑可以更复杂更灵活
*/
