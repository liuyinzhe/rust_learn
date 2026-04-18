
// cargo add tokio -F full
// cargo add async-recursion
// cargo run --example example18

// use async_recursion::async_recursion;

use std::pin::Pin;
use std::future::Future;




#[tokio::main]
async fn main() {
    // println!("fibonacci(10) = {}", fibonacci(10));
    // println!("fibonacci(10) = {}", fibonacci(10).await);

    // let future = async {
    //     println!("Hello");
    // };
    // // future.await;
    // tokio::pin!(future); // 内存地址订住 // 利用变量遮蔽 Shadowing // unsafe 担保
    // (&mut future).await;

    let task = get_task("db");
    let result = task.await;
    println!("Got result : {result}");
}


// fn fibonacci(n: u32) -> u32 {
//     match n {
//         0 => 0,
//         1 => 1,
//         _ => fibonacci(n - 1) + fibonacci(n - 2),
//     }
// }


// #[async_recursion]
// async fn fibonacci(n: u32) -> u32 {
//     match n {
//         0 => 0,
//         1 => 1,
//         _ => fibonacci(n - 1).await + fibonacci(n - 2).await,
//     }
// }

fn get_task(source: &str) -> Pin<Box<dyn Future<Output = String>>> {
    match source {
        "db" => Box::pin(from_db()),
        "api" => Box::pin(from_api()),
        _ => Box::pin(async { "Unknown source".to_string() }),
    }
}

async fn from_db() -> String {
    "DB data".to_string()
}

async fn from_api() -> String {
    "API data".to_string()
}