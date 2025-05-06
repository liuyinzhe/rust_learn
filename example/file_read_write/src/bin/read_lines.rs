
use std::{fs::File, io::{BufRead, BufReader, Write}};
fn main(){
    let file = File::open("./data/a.txt").unwrap();
    let reader = BufReader::new(file);

    let mut out_file = File::create("./data/b.txt").unwrap();

    for line in reader.lines(){//.take(10){
        let line = line.unwrap();
        //println!("{}",line);
        let line = format!("{}\n",line);
        out_file.write_all(line.as_bytes()).unwrap();
    }
    println!("done");
}
