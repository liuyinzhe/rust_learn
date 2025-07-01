
use std::{error::Error, fs, path::PathBuf};
use regex::Regex;
use rfd::FileDialog;
use serde::Deserialize;
use serde::Serialize;
pub fn read_txt_to_json(file_path:&std::path::PathBuf) -> Result<PathBuf,Box<dyn Error>> {
    //todo!()
    let txt = fs::read_to_string(file_path)?;

    let mut disc_no = 0u32;
    let disc_regex = Regex::new(r"^(\d+)\.$").unwrap(); // Result<Regex, Error> 所以要加 unwrap()
    let movie_regex= Regex::new(r"^(\d{4})(.*?)(（儿童）)?$").unwrap();//?//.unwrap()
    let mut movies = Vec::new();
    for line in txt.lines().map(str::trim).filter(|l|!l.is_empty()) {
        // 若解析成功，将返回的碟片编号 no 赋值给外部变量 disc_no
        if let Some(no) = disc_number(line,&disc_regex){
            //println!("{no:#?}");
            disc_no = no;
        } else { //当行不是碟片编号行时，尝试将其解析为电影信息
            if let Some(movie) = parse_movie(disc_no, line,&movie_regex){
                movies.push(movie);
            }
        }
    }
    // Ok(PathBuf::new())
    save_to_json(movies)
}

fn save_to_json(movies:Vec<Movie>) -> Result<PathBuf, Box<dyn Error>> {
    // cargo add serde -F derive // features = {"derive"} // 单纯框架
    // cargo add serde_json
    let json_str = serde_json::to_string_pretty(&movies)?;
    let path = FileDialog::new()
        .add_filter("JSON",&["json"])
        .set_title("Save data to JSON file")
        .set_directory(r"/Users/dave/Desktop")
        .save_file()
        .ok_or_else(|| "No save location selected".to_string())?;
    fs::write(&path, json_str)?;
    Ok(path)
}

fn parse_movie(disc_no: u32, line:&str, re:&Regex)  -> Option<Movie> {
    re.captures(line).map(|caps: regex::Captures<'_>|{ // map(||) 闭包操作
        println!("{caps:#?}");
        Movie { 
            disc: disc_no, 
            year: caps.get(1).unwrap().as_str().trim().to_string(), 
            title: caps.get(2).unwrap().as_str().trim().to_string(), 
            remark: caps.get(3).map(|m|m.as_str().trim().to_string())// 不一定有,Option 
        }
    })

}

fn disc_number(line: &str, re: &Regex) -> Option<u32> {
    // if let Some(caps) = re.captures(line) {
    //     println!("Caps: {caps:#?}");
    //     /*
    //         Caps: Captures(
    //             {
    //                 0: 0..4/"120.", //第一项 坐标范围 全体匹配
    //                 1: 0..3/"120",  //第二项 坐标范围 捕获匹配
    //             },
    //         )
    //     */
    //     caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
    // }
    // Some(0u32)

    // if let Some(caps) = re.captures(line) {
    //     Some(caps.get(1).unwrap().as_str().parse::<u32>.unwrap())
    // }else{
    //     None


    re.captures(line)
        .map(|caps|caps.get(1).unwrap().as_str().parse::<u32>().unwrap())

}

#[derive(Debug,Serialize,Deserialize)]
struct Movie {
    disc: u32,
    year: String,
    title: String,
    remark: Option<String>
}
