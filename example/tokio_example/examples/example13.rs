use std::{
    fs::File,
    // BufRead trait，它提供了 lines() 方法，用于按行迭代读取内容
    io::{self,BufRead}, // BufRead trait is required for lines() method
    path::{Path, PathBuf}, // Path 是路径的不可变视图, PathBuf 是可拥有的路径结构，常用于泛型约束
};
// 异步版本
use tokio::time::Instant;
use tokio::time::{sleep,Duration};
// cargo add anyhow
// cargo add tokio -F full

// cargo run --example example13 
#[tokio::main]
async fn main() {
    // println!("Hello, world!");
    // let _ = read_lines("a.txt");
    // let _ = read_lines("a.txt".to_string());
    // let _ = read_lines(String::from("a.txt"));
    // let _ = read_lines(Path::new("a.txt"));
    // let _ = read_lines(PathBuf::from("a.txt"));
    let _ = count_lines_sync();

    let new = Instant::now();
    let line_count = count_lines_sync();
    println!(
        "Sync version read {} lines in {:.3} seconds",
        line_count,
        new.elapsed().as_secs_f32() // 返回从 Instant 创建到现在的时间间隔 Duration
        // 将 Duration 转换为秒的 f32 浮点数，方便打印
             );
    
    // Fake version
    let _ = tokio::join!( // 宏，允许同时等待多个异步任务; 注意并发不等于并行
        count_lines_sync_fake("Shakespeare.txt".to_string()),
        count_lines_sync_fake("Shakespeare.txt".to_string()),
        // ticker(),// 真的异步加了await
    );
    println!(
        "Fake async version all: {:.3} seconds",
        new.elapsed().as_secs_f32()
    );

    // async version
    let _ = tokio::join!(
        count_line_async("Shakespeare.txt".to_string()),
        count_line_async("Shakespeare.txt".to_string()),
        ticker(),// 真的异步加了await
    );
    println!(
        "Fake async version all: {:.3} seconds",
        new.elapsed().as_secs_f32()
    );

}

fn count_lines_sync() -> i32 {
    let mut count = 0;
    // 	模式匹配, 取值后 变量为lines
    if let Ok(lines) = read_lines("Shakespeare.txt") {
        // Calls a closure on each element of an iterator
        // lines 是实现了 Iterator<Item = Result<String, io::Error>> 的类型，
        // for_each 对每个元素执行闭包
        lines.for_each(|line|{
            // let-else 的变体写法，等价于先判断 line.is_ok() 再取 line.unwrap()
            // && 检查去除首尾空白后是否为空
            if let Ok(line) = line && !line.trim().is_empty() {
                count += 1;
            } 
        });
    }
    count
}

// 
fn read_lines<T>(filename: T) -> anyhow::Result<io::Lines<io::BufReader<File>>>
where // 声明泛型约束
    // 等价于fn read_lines<T: AsRef<Path>>(filename: T) -> anyhow::Result<...>
    T: AsRef<Path>,
    // T: AsRef<Path> 泛型参数，因此可以传入多种能转换为路径的类型：&str、String、&Path、PathBuf 等
    // 这是 Rust 泛型约束 AsRef<Path> 的灵活体现
{
    let file = File::open(filename)?; // ?异常则直接中断
    // 创建带缓冲的读取器，lines() 返回一个迭代器，该迭代器每次迭代产生 Result<String>
    Ok(io::BufReader::new(file).lines())
}

async fn count_lines_sync_fake(filename: String) -> anyhow::Result<usize> {
    // 该函数虽为 async，但因无 .await，运行时在执行时不会让出控制权，仍会阻塞线程。
    // 因此 tokio::join! 并发调用它们时并不会真正并行，而是顺序执行。
    println!("Start executing fake count_line_async");
    let new = std::time::Instant::now();
    let mut count = 0;

    if let Ok(lines) = read_lines(filename) {
        // Calls a closure on each element of an iterator
        lines.for_each(|line|{
            if let Ok(line) = line 
            && !line.trim().is_empty() 
            {
                count += 1;
            } 
        });
    }
    println!(
        "Fake version read {} lines in {:.3} seconds",
        count,
        new.elapsed().as_secs_f32() 
    );
    Ok(count)
}


async fn ticker() {
    for _ in 0..30 {
        println!("tick");
        // tokio::time::sleep 异步版,可await, 让出控制权,允许其他任务执行 
        sleep(Duration::from_millis(5)).await;
    }
}

async fn count_line_async(filename: String) -> anyhow::Result<usize> {
    use tokio::fs::File;    // 替代 std::fs::File，适用于异步文件操作
    use tokio::io::BufReader; // 替代 std::io::BufReader，适用于异步文件操作
    use tokio::io::AsyncBufReadExt; // lines() method // AsyncBufRead trait is required for lines() method
    // 替代 std::io::BufRead，提供了 next_line() 方法用于异步按行读取
    println!("Start async version reading: {filename}...");
    let new = std::time::Instant::now();
    let mut count = 0 ;
    let file = File::open(filename).await?; // 打开文件时,可以让出控制权,允许其他任务执行
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? { // 逐行读取时,可以让出控制权,允许其他任务执行
        if !line.trim().is_empty() {
            count += 1;
        } 
    }

    println!(
        "Async version read {} lines in {:.3} seconds",
        count,
        new.elapsed().as_secs_f32() 
    );
    Ok(count)
}
