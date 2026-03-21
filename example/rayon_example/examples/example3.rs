// examples/example3.rs
// cargo run --example example3
fn main() {
    // 广播,所有任务都执行同一个函数
    let pool = rayon::ThreadPoolBuilder::new()
    .num_threads(4)
    .build()
    .unwrap();

    pool.scope(|scope|{
        // 闭包接受两个参数：第一个是 &Scope（通常用下划线忽略），第二个是 BroadcastContext，
        // 它提供了当前线程的上下文信息。

        /*
        方法	行为	适用场景
        scope.spawn	将任务提交给线程池，由某个空闲线程执行（不保证每个线程都执行）	一般并行计算
        scope.spawn_broadcast	线程池中每个线程都执行一次闭包	每个线程独立初始化、收集线程局部信息
        */
        scope.spawn_broadcast(|_scope,ctx|{
            let id = ctx.index();
            // 多个线程同时调用 println! 是安全的，因为 println! 内部使用了互斥锁，输出不会交错损坏。
            println!("Thead {id}.");
        });
    });
}