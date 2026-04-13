
use std::{
    fs::File,
    io::{self,BufRead}, // BufRead trait is required for lines() method
    path::{Path, PathBuf},
};
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
        new.elapsed().as_secs_f32() 
             );
    
    // Fake version
    let _ = tokio::join!(
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
    if let Ok(lines) = read_lines("Shakespeare.txt") {
        // Calls a closure on each element of an iterator
        lines.for_each(|line|{
            if let Ok(line) = line && !line.trim().is_empty() {
                count += 1;
            } 
        });
    }
    count
}

// 
fn read_lines<T>(filename: T) -> anyhow::Result<io::Lines<io::BufReader<File>>>
where
    T: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

async fn count_lines_sync_fake(filename: String) -> anyhow::Result<usize> {
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
        sleep(Duration::from_millis(5)).await;
    }
}

async fn count_line_async(filename: String) -> anyhow::Result<usize> {
    use tokio::fs::File;
    use tokio::io::BufReader;
    use tokio::io::AsyncBufReadExt; // lines() method // AsyncBufRead trait is required for lines() method

    println!("Start async version reading: {filename}...");
    let new = std::time::Instant::now();
    let mut count = 0 ;
    let file = File::open(filename).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
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
