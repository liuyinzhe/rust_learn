// main.rs
use libloader::libloading; // 首先需要引用libloader的libloading,侧面印证了libloader是基于libloading的
use libloader::get_libfn;
fn main() {
    get_libfn!("/mnt/c/Users/Unicorn/Desktop/test/rust/lib/rust_lib/libmydynamiclinklibrary.so", "println", my_println, (), str: &str); // 获取dll的函数
    //          ^链接库路径                                                                       ^库中的函数 ^调用的名称 ^返回值   ^参数
    my_println("Hello World");

    get_libfn!("/mnt/c/Users/Unicorn/Desktop/test/rust/lib/rust_lib/libmydynamiclinklibrary.so", "add", my_add, usize, a: usize, b: usize);
    println!("10 + 20 = {}", my_add(10, 20));

    get_libfn!("/mnt/c/Users/Unicorn/Desktop/test/rust/lib/rust_lib/libmydynamiclinklibrary.so", "print_hello", my_print_hello, ());
    my_print_hello();
}