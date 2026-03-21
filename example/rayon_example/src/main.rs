use std::time::Instant;
use rayon::prelude::*;

// cargo add rayon

// cargo run --bin rayon_example
fn is_prime(n:u64) -> bool {
    //into_par_iter()// Converts self into a parallel iterator.
    //all()// Tests that every item in the parallel iterator matches the given predicate, 
    // and if so returns true. If a counter-example is found, 
    // we'll attempt to stop processing more items, then return false.
    // all 遍历所有符合闭包条件,则返回true
    
    //(2..=n / 2).into_par_iter().all(|i| n % i !=0 ) // par_iter(),par_iter_mut(), into_par_iter()

    if n < 2 { return false; }
    let limit = (n as f64).sqrt() as u64; // n/2 是正确且安全的，只是效率较sqrt()低。
    (2..=limit).all(|i| n % i != 0)

    /*
    方法	适用场景	是否消耗原数据	迭代元素类型
    par_iter()	对集合的不可变引用进行并行迭代	否	&T
    par_iter_mut()	对集合的可变引用进行并行迭代	否	&mut T
    into_par_iter()	消耗集合本身，取得所有权	是	T（对于集合）或直接是元素（对于范围）
    */
}

fn main() {
    let now = Instant::now();
    let nums: Vec<u64> = (2..10000).collect(); // 生成 2~9999 的向量
    let mut primes: Vec<&u64> = nums.par_iter()
    // 过滤出素数
    .filter(|&n| is_prime(*n as u64))
    .collect(); 
    let elapsed = now.elapsed();
    primes.par_sort_unstable(); // par_sort()（稳定）和 par_sort_unstable()（不稳定
    // println!("{primes:?}");
    println!("{} ms to find {} primes",elapsed.as_millis(),primes.len());
}
