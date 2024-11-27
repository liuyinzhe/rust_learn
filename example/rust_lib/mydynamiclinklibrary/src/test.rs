// lib.rs // RUS中函数或静态变量使用#[no_mangle]这个标注属性后，编译器就不会修改它们的名字了。
#[no_mangle]
pub fn println(str: &str) { // 有参数没有返回值
    println!("{}", str);
}

#[no_mangle]
pub fn add(a: usize, b: usize) -> usize { // 有参数有返回值
    a + b
}

#[no_mangle]
pub fn print_hello() { // 没有参数没有返回值
    println!("Hello");
}