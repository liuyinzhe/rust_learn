use rayon::prelude::*;
use std::time::SystemTime;

// examples/example5.rs
// cargo run --example example5
fn main() {
    // rayon利用 task stealing 调度策略, 减少线程之间负载不平衡，提高整体的并行效率
    let arr: Vec<i64> = (1..=1000_0000).collect();

    let time_a = SystemTime::now();

    let result = sum_of_add(&arr);
    println!("{}",result);

    let time_b = SystemTime::now().duration_since(time_a).unwrap();
    println!("{:?}",time_b);
}

fn sum_of_add(input:&[i64]) -> i64 {
    // input.iter() // 84.0081ms
    input.par_iter() //parallel // 18.6151ms
    .map(|&i| i + i )
    .sum()
}