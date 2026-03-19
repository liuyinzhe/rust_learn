// examples/example2.rs
// cargo run --example example2
fn main() {
    // 多个线程池,多阶段并行处理
    let outer_pool = rayon::ThreadPoolBuilder::new()
    .num_threads(4)
    .build()
    .unwrap();

    outer_pool.scope(|scope|{
        for stage in 0..3 {
            scope.spawn(move |_scope|{
                println!("Stage {stage} started.");

                let inner_pool = rayon::ThreadPoolBuilder::new()
                .num_threads(2)
                .build()
                .unwrap();
                inner_pool.scope(|inner_scope|{
                    for task in 0..2 {
                        inner_scope.spawn(move|_inner_scope|{
                            println!("\t-> Inner task {task} of stage {stage}");
                            
                        });
                    }
                });

                println!("Stage {stage} completed.");
            });
        }
    });
    println!("All stages complete.")
}