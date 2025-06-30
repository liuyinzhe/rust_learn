
use std::{error::Error, fs, path::PathBuf};
use regex::Regex;
pub fn read_txt_to_json(file_path:&std::path::PathBuf) -> Result<PathBuf,Box<dyn Error>> {
    //todo!()
    let txt = fs::read_to_string(file_path)?;

    let mut disc_no = 0u32;
    let disc_regex = Regex::new(r"^(\d+)\.$").unwrap(); // Result<Regex, Error> 所以要加 unwrap()
    for line in txt.lines().map(str::trim).filter(|l|!l.is_empty()) {
        if let Some(no) = disc_number(line,&disc_regex){
            println!("{no:#?}");
            disc_no = no;
        }
        // }else {
        //     if let Some(movie) = parse_movie()
        // }


    }
    Ok(PathBuf::new())
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

struct Movie {
    disc: u32,
    year: String,
    title: String,
    remark: Option<String>
}