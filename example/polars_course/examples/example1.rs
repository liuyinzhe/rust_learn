use polars::prelude::*;


// cargo run --example example1

fn main() -> PolarsResult<()> {
    // 1. 构建 LazyFrame（查询计划）
    let lf = df![
        "name" => ["Alice", "Bob", "Charlie"],
        "age" => [25, 30, 35],
        "city" => ["New York", "San Francisco", "Seoul"],
    ]?.lazy();// 转为 LazyFrame

    // 2. 链式构建查询（还没真正执行）
    let q = lf.filter(col("age").gt(lit(30)))
        .select([col("name"), col("city")])
        .sort(["name"], SortMultipleOptions::default());
    /*
        filter(col("age").gt(lit(30)))
        col("age")：创建一个列表达式 Expr，代表名为 "age" 的列
        lit(30)：创建一个字面量表达式，值为整数 30
        .gt(lit(30))：gt 即 "greater than"，产生一个布尔表达式 age > 30
        filter(...)：接收一个布尔表达式，返回新的 LazyFrame，表示过滤操作

        .select([col("name"), col("city")])
        作用：选择最终结果中保留的列。此处仅保留 "name" 和 "city"
        select 方法：接受一个或多个表达式（这里是列表达式），返回仅含所选列的新 LazyFrame

        .sort(["name"], SortMultipleOptions::default())
        作用：按 "name" 列升序排序
        SortMultipleOptions::default()：提供默认排序选项（升序、空值置后、单线程排序等）。可通过构建器模式自定义排序行为，例如：
        
        SortMultipleOptions::new().with_order_descending(true)
    */

    // 3. 执行并收集结果
    let df = q.clone().collect()?;
    /*
        q.clone()：由于 .collect() 会消耗 self（所有权移动），
        而我们后面还需要用 q 来解释查询计划，因此先克隆一份
        克隆 LazyFrame 仅复制查询计划的轻量级描述，成本很低
    */
    println!("lazy 查询结果：");
    println!("{:?}", df);
    // 4. 查看查询计划（超级实用！）
    println!("\n 查询计划");
    println!("{:?}", q.explain(false)?);
    /*
        explain(optimized: bool)：返回一个描述查询计划的字符串
        若 optimized = false：显示原始逻辑计划（用户指定的操作序列）
        若 optimized = true：显示优化后的逻辑计划（经过谓词下推、投影下推等规则重写后的计划）
    */

    Ok(())

}