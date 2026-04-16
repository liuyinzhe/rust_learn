// cargo add polars -F "lazy,csv,parquet,json,dtype-full,performant"

use polars::prelude::*;

fn main()-> PolarsResult<()> {
    let df = df! {
        "name" => ["Alice", "Bob", "Charlie"],
        "age" => [25, 30, 35],
        "city" => ["New York", "London", "Paris"],
    }?;
    println!("Dataframe 预览");
    println!("{}", df);

    println!("\nSchema");
    println!("{:?}", df.schema());

    println!("\n前 2 行");
    println!("{:?}", df.head(Some(2)));

    Ok(())
}
