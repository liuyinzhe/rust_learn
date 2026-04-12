

// cargo add tokio -F full
// cargo add futures
// cargo add anyhow
// cargo add thiserror

// cargo run --example example12 // 错误处理(3)
/*
    Async 错误处理
*/


use tokio::time::sleep;
use std::time::Duration;

/*
#[tokio::main] 展开后相当于：
fn main() {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async { /* 原来的 async main 内容 */ });
}
*/
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // todo!();
    // fetch_user(3).await?;

    let mut tasks = Vec::new();
    for id in 1..=10 {
        tasks.push(fetch_user(id));
        // fetch_user(id) 是一个异步函数调用，但它并不会立即执行，
        //而是返回一个实现了 Future 的值（一个惰性的异步计算）
    }

    /*
    join_all 来自 futures 库（Tokio 也导出了类似的 tokio::try_join! 等工具，但 join_all 更通用）。
    它接受一个 Future 的集合，并发地运行它们，返回一个 Vec 包含每个 Future 的完成结果（顺序与输入顺序一致）。
    join_all 会等待所有 Future 完成才返回。如果某个 Future 失败，它依然会等待其他 Future 完成，
    并将失败的结果包含在最终 Vec 中。

    */
    let results = futures::future::join_all(tasks).await;
    println!("{:?}", results);
    // [Ok("User_1"), Ok("User_2"), Err(User 3 does not exist), Ok("User_4"), Ok("User_5"), Ok("User_6"), Ok("User_7"), Ok("User_8"), Ok("User_9"), Ok("User_10")]
    
    // // into_iter() 将结果全部转换为anyhow::Result<Vec<String>> 类型
    // let final_result: anyhow::Result<Vec<String>> = results.into_iter().collect();
    // let users = final_result?;  // ? 出错就停止程序
    // // ["User_1", "User_2", "User_3", "User_4", "User_5", "User_6", "User_7", "User_8", "User_9", "User_10"]
    // println!("{users:?}");


    // 有错误依然进行，但是记录下错误
    let mut errors = Vec::new();
    /*
    into_iter
        将 Vec<Result<String, anyhow::Error>> 转换为所有权迭代器，消耗 results，产生每个 Result
    filter_map
        filter_map 接受一个闭包，对每个元素：
        如果闭包返回 Some(value)，则 value 被加入结果迭代器；
        如果返回 None，则跳过该元素。
        这里我们想：成功时保留值（Some），失败时记录错误但不保留值（None）
    
    闭包r.map_err(|e| errors.push(e)).ok()
        r 是 Result<String, anyhow::Error>。
        .map_err(|e| errors.push(e))：// 该表达式返回 ()（单元类型），因为 push 的返回值是 ()
            如果 r 是 Err(e)，则调用闭包，将错误 e 推入 errors 向量，并返回一个新的 Result<String, ()>（错误类型变为单元类型 ()，但错误值已被丢弃）。
            如果 r 是 Ok(v)，则原样返回 Ok(v)。
        .ok()：
            将 Result<T, E> 转换为 Option<T>：
            Ok(v) → Some(v)
            Err(_) → None // 丢弃错误值
        由于 map_err 后错误类型变为 ()，ok() 会丢弃单元错误，得到 Option<String>。
        因此，整个闭包的效果是：
            成功时：返回 Some(String)，filter_map 将其纳入 ok_users。
            失败时：先将原始错误记录到 errors 中，然后返回 None，filter_map 跳过它。

        .filter_map(|r| 
            r.map_err(|e| errors.push(e)).ok()
        )
        r: Result<String, anyhow::Error>
                │
                ├── Ok(s) ─────────────────► Ok(s) ──► Some(s) ──► 进入 ok_users
                │
                └── Err(e) ──► map_err(|e| errors.push(e)) ──► Err(())
                                                                    │
                                                                    ▼
                                                                ok() 返回 None ──► 被 filter_map 跳过
    map_err 消耗了 r（获取了 r 的所有权），并把 e 传给闭包。
    闭包内 errors.push(e) 将 e 的所有权移入 errors。
    闭包返回 ()，被 map_err 用来构造新的 Err(())。
    ok() 消耗这个 Err(())，丢弃 ()，返回 None。
    Result<T,E> 传输的内容时 Some(T) 或者Err(E)
    map_err 的语义是：
    如果输入是 Ok(t)，则直接返回 Ok(t)（成功值原封不动透传）。
    如果输入是 Err(e)，则用闭包处理 e，返回一个新的错误值，包装在 Err 里。
    闭包处理后|e| errors.push(e) 为 Err(())
    传给 ok() 的是 [Some(T),Err(())] Result<T,E> 类型的列表

    ok() 将Result<T,E> 转为Option<T>
        Ok(s) → Some(s)
        Err(()) → None
    同时闭包外层的filter_map 会过滤掉 None 值，只保留 Some(s) 值
    */
    let ok_users: Vec<_> = results
        .into_iter() 
        .filter_map(|r|  // 过滤
            r.map_err(|e| errors.push(e)).ok()) // 映射错误内容，错误才存入errors,ok()保留
        .collect();// ok列表收集



    println!("OK: {ok_users:?}");
    println!("Errors: {errors:?}");
    Ok(())
}

async fn fetch_user(id: u32) -> anyhow::Result<String> {
    if id == 3 {
        anyhow::bail!("User {id} does not exist"); // 等价于 return Err(anyhow::anyhow!(...))，用于提前返回错误
    }
    sleep(Duration::from_secs(1)).await;
    /*
    // use tokio::time::sleep;
    sleep(Duration::from_secs(1)).await：异步等待 1 秒，期间 Tokio 可以调度其他任务。
    标准库的 std::thread::sleep 会阻塞当前线程，不能用于异步代码。
    */
    Ok(format!("User_{id}"))
}