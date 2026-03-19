// examples/example3.rs
// cargo run --example example3
fn main() {
    // 广播,所有任务都执行同一个函数
    let pool = rayon::ThreadPoolBuilder::new()
    .num_threads(4)
    .build()
    .unwrap();

    pool.scope(|scope|{
        scope.spawn_broadcast(|_scope,ctx|{
            let id = ctx.index();
            println!("Thead {id}.");
        });
    });
}