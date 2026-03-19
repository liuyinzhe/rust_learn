use std::time::Instant;
use rayon::prelude::*;

// cargo add rayon

fn is_prime(n:u32) -> bool {
    (2..=n / 2).into_par_iter().all(|i| n % i !=0 ) // par_iter(),par_iter_mut(), into_par_iter()
}

fn main() {
    let now = Instant::now();
    let nums: Vec<u64> = (2..10000).collect();
    let mut primes: Vec<&u64> = nums.par_iter()
    .filter(|&n| is_prime(*n as u32))
    .collect(); 
    let elapsed = now.elapsed();
    primes.par_sort_unstable();
    // println!("{primes:?}");
    println!("{} ms to find {} primes",elapsed.as_millis(),primes.len());
}
