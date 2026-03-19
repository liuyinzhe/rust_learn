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
        for (i,row) in matrix.iter().enumerate() {
            scope.spawn(move |_| {
                let sum: i32 = row.iter().sum();
                println!("Row {i} sum= {sum}");
            });
        }
    });
    println!("main thread finished");
}