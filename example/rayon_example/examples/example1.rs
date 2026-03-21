// examples/example1.rs
// cargo run --example example1
fn main() {
    let pool = rayon::ThreadPoolBuilder::new()
    .num_threads(4)
    .build()
    .unwrap();

    let matrix = [
        vec![1,2,3],
        vec![4,5,6],
        vec![7,8,9],
        vec![10,11,12],
    ];

    pool.scope(|scope|{
        /*
        scope 会阻塞当前线程，直到所有子任务完成
        这个 Scope 对象允许我们在该作用域内派生（spawn）任务

        matrix.iter() 返回一个迭代器，产生数组中的每个元素（即 &Vec<i32>）
        enumerate() 将迭代器包装成产生 (index, value) 对的迭代器，i 是行索引（从 0 开始），
        row 是每行的引用。
        */
        for (i,row) in matrix.iter().enumerate() {
            scope.spawn(move |_| { // 任务作用域
                let sum: i32 = row.iter().sum();
                println!("Row {i} sum= {sum}");
            });
        }
    });
    println!("main thread finished");
}