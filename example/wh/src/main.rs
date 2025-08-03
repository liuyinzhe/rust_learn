use std::{sync::atomic::AtomicUsize, thread, time::Instant};

use rand::Rng;

// cargo run --bin wh
fn main() {
    let start = Instant::now();
    
    // products, 1,2,3,4,5
    let section_count = rand::rng().random_range(10..=20);// 10-20任意数字
    let mut sections  = Vec::new();
    let mut actual = [0;5]; // 固定大小数组
    for _ in 0..section_count{
        let mut section = Section([0;5]);
        for (i,p)  in section.0.iter_mut().enumerate(){ //enumerate() (index, item)
            *p = rand::rng().random_range(0..=1_000_000);
            actual[i] += *p;
        }
        sections.push(section);
    }
    println!("Actual: {actual:#?}");

    let counted: [AtomicUsize;5] = Default::default();// 初始化为零值
    thread::scope(|s|{
        for sec in sections.iter() {
            s.spawn(||{ // 每个线程
                for (i,c) in sec.0.iter().enumerate(){ // (())
                    for _ in 0..*c {
                        // 隐式使用元素 fetch_add 参数是引用
                        counted[i].fetch_add(1, std::sync::atomic::Ordering::Relaxed); //fetch_add 无锁原子加1操作
                    }
                }

            });
        }
    });
    println!("Counted: {counted:#?}");

    for i in 0..5{
        assert_eq!(actual[i],counted[i].load(std::sync::atomic::Ordering::Relaxed));
    }
    let elapsed = start.elapsed();
    println!("Elapsed time: {}", elapsed.as_micros());
}

struct  Section([usize;5]);