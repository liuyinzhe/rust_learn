/*
    高级Trait

    在trait 定义中使用关联类型指定占位符类型

    关联类型

    关联类型将一个 "类型占位符" 绑定到trait 中
    允许 trait 的方法使用这些 占位类型
    实现该trait时, 具体实现者提供实际的类型

*/

// 示例
pub trait Iterator {
    type Item; // 关联类型，表示同一种类型
    
    fn next(&mut self) -> Option<self::Item>;
}

// 关联类型  vs 泛型参数

//关联类型
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
