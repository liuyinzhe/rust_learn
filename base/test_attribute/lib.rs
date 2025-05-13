    // 设置所需的数据或者装填
    // 运行需要被测试的代码
    // 断言其结果是你想要的
    /*
    测试就是带有 #[test] 属性(attribute)的函数
    cargo test 
    ----> test runner(二进制)
    运行带有#[test] 的函数，并报告结果
     */
#![allow(unused)]  // 允许代码中存在未使用的变量/函数而不触发警告 //等价写法：#[allow(unused_variables, unused_mut)]

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self,other:&Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}



pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(a: usize) -> usize {
    a + 2
}

pub fn greeting(name:&str) ->String {
    format!("Hello {name}!")
}

pub struct Guess{
    value:i32,
}

impl Guess {
    pub fn new(value:i32) ->Guess{
        // 创建时的检查
        if value <1 {
            panic!("Guess value mast be greater than or equal to 1,get {value}.");
        } else if value > 100 {
            panic!("Guess value mast be less than or equal to 100,get {value}.");
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*; // 使用mod 模块外的结构体和函数;进行引入，才能调用

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4); // 断言 是否相等;接受boolean 作为参数
    }

    #[test]
    fn it_works() -> Result<(),String> { // 对于返回值为 Result<T,E> 泛型 Ok(),Err(); 使用 assert!(value.is_err()) 进行断言
        let result = add(2, 2);
        if result == 4{
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn test_it_works(){
        //assert!(!it_works().is_err());// ok
        assert!(it_works().is_err()); // failed
    }


    #[test]
    fn another(){
        panic!("Make this test fail");
    }

    
    #[test]
    fn larger_can_hold_samaller(){
        let larger= Rectangle{
            width:8,
            height:7,
        };
        let smaller = Rectangle{
            width:5,
            height:1,
        };
        assert!(larger.can_hold(&smaller));
        
    }

    #[test]
    fn smaller_cannot_hold_larger(){
        let larger= Rectangle{
            width:8,
            height:7,
        };
        let smaller = Rectangle{
            width:5,
            height:1,
        };
        assert!(!smaller.can_hold(&larger)); // ! 叹号否定
        
    }
    /*
    # 断言 相比assert!() 会打印出数值
    assert_eq!() 不相等 == 
    assert_ne!() 不相等 !=
     */

    #[test]
    fn it_adds_two(){
        let result = add_two(2);
        assert_eq!(result,5); // 断言失败，则输出具体数值
        /*
        thread 'tests::it_adds_two' panicked at src\lib.rs:84:9:
        assertion `left == right` failed
        left: 4
        right: 5
         */
    }

    #[test]
    fn greeting_contains_name(){
        let result = greeting("Carol");
        assert!(result.contains("Carol"), // 第二个是自定义报错内容
                "Greeting did not contain name, value wes `{}`",result); // 字符串包含
    }


    #[test]
    //#[should_panic] // 断言 发生panic
    #[should_panic(expected = "less than or equal to 100")] // 匹配panic的输出信息,来测试指定的panic条件
    fn greater_than_100(){
        Guess::new(200);
    }
}
