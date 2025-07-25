use std::{collections::HashMap, error::Error, fs, time::Instant};

fn main() -> Result<(),Box<dyn Error>> {
    // 计时
    let start = Instant::now();
    // 单线程
    let path = r"C:\Users\DV\Desktop\test\rust\counter\files";
    let mut map = HashMap::new();
    fs::read_dir(path)?
    .filter_map(|entry|{
        let entry = entry.ok()?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s|s.to_str()) == Some("txt"){
            Some(path)
        }else{
            None
        }
    })
    .for_each(|p|{
        if let Ok(text) = fs::read_to_string(p) {
            text.split_whitespace().for_each(|w|{
                let word = w
                    .trim_matches(|c:char|c.is_ascii_punctuation())
                    .to_lowercase();
                if !word.is_empty() {
                    // 解引用
                    *map.entry(word).or_insert(0) += 1;
                }

            });
        }
    });

    println!("Map count: {}",map.len());
    let mut vec: Vec<_> = map.iter().collect();
    // 排序前10
    vec.sort_by(|a,b| b.1.cmp(a.1));
    
    for i in 0..10{
        println!("{}: {}", vec.get(i).unwrap().0,vec.get(i).unwrap().1);
    }
    let elapsed= start.elapsed();
    println!("Time elapsed: {}",elapsed.as_micros());// 毫秒
    Ok(())
}
