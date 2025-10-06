use ansi_term::Style;       // 终端样式和颜色
use anyhow::{bail, Result};  // bail!()包装错误 // Result 类型
use chrono::{Datelike, Local, NaiveDate};  // 日期处理
use clap::Parser;
use itertools::izip;  // 与标准库的 zip 相比,izip! 允许一次合并多于两个的迭代器

// cargo add  clap --features derive
// cargo add ansi_term anyhow chrono itertools

// cargo add --dev assert_cmd predicates pretty_assertions rand sys_info


#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `cal`
struct Args {
    /// Year (1-9999)
    #[arg(value_parser(clap::value_parser!(i32).range(1..=9999)))]
    year: Option<i32>,

    /// Month name or number (1-12)
    #[arg(short)]
    month: Option<String>,

    /// Show the whole current year
    #[arg(short('y'), long("year"), conflicts_with_all(["month", "year"]))]
    show_current_year: bool,  // 显示当前年份标志
}

const LINE_WIDTH: usize = 22;
const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

// --------------------------------------------------
fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

// --------------------------------------------------
fn run(args: Args) -> Result<()> {
    let today = Local::now().date_naive(); // 获取当前日期
    let mut month = args.month.map(parse_month).transpose()?; // 解析月份
    let mut year = args.year;

    if args.show_current_year {
        // 显示当前年份：清空月份，设置年份为当前年
        month = None;
        year = Some(today.year());
    } else if month.is_none() && year.is_none() {
        // 无参数：显示当前月份
        month = Some(today.month());
        year = Some(today.year());
    }
    let year = year.unwrap_or(today.year());

    match month {
        Some(month) => {
            let lines = format_month(year, month, true, today);
            println!("{}", lines.join("\n"));
        }
        None => {
            println!("{year:>32}");
            let months: Vec<_> = (1..=12)
                .map(|month| format_month(year, month, false, today))
                .collect();

            for (i, chunk) in months.chunks(3).enumerate() {
                if let [m1, m2, m3] = chunk {
                    for lines in izip!(m1, m2, m3) {
                        println!("{}{}{}", lines.0, lines.1, lines.2);
                    }
                    if i < 3 {
                        println!();
                    }
                }
            }
        }
    }

    Ok(())
}

// --------------------------------------------------
fn parse_month(month: String) -> Result<u32> {
    match month.parse() {
        Ok(num) => {
            // 数字解析成功，检查范围
            if (1..=12).contains(&num) {
                Ok(num)
            } else {
                bail!(r#"month "{month}" not in the range 1 through 12"#)
            }
        }
        _ => {
            // 尝试按月份名称解析
            let lower = &month.to_lowercase();
            let matches: Vec<_> = MONTH_NAMES
                .iter()
                .enumerate()
                .filter_map(|(i, name)| {
                    if name.to_lowercase().starts_with(lower) {
                        Some(i + 1)
                    } else {
                        None
                    }
                })
                .collect();

            if matches.len() == 1 {
                Ok(matches[0] as u32)
            } else {
                bail!(r#"Invalid month "{month}""#)
            }
        }
    }
}

// --------------------------------------------------
fn last_day_in_month(year: i32, month: u32) -> NaiveDate {
    // 计算下个月的第一天
    // The first day of the next month...
    let (y, m) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };

    // ...is preceded by the last day of the original month
    NaiveDate::from_ymd_opt(y, m, 1)
        .unwrap()
        .pred_opt() // 获取前一天的日期
        .unwrap()
}

// --------------------------------------------------
fn format_month(
    year: i32,
    month: u32,
    print_year: bool,
    today: NaiveDate,
) -> Vec<String> {
    let first = NaiveDate::from_ymd_opt(year, month, 1).unwrap();

    // 生成日期前的空白（对齐星期）
    let mut days: Vec<String> = (1..first.weekday().number_from_sunday())
        .map(|_| "  ".to_string()) // two spaces
        .collect();

    // 检查是否是今天的日期
    let is_today = |day: u32| {
        year == today.year() && month == today.month() && day == today.day()
    };

    // 添加所有日期，高亮今天
    let last = last_day_in_month(year, month);
    days.extend((first.day()..=last.day()).map(|num| {
        let fmt = format!("{num:>2}"); // 右对齐两位数字
        if is_today(num) {
            Style::new().reverse().paint(fmt).to_string() // 反色显示今天
        } else {
            fmt
        }
    }));

    let month_name = MONTH_NAMES[month as usize - 1];
    let mut lines = Vec::with_capacity(8); // 预分配空间,8行

    // 月份标题行
    lines.push(format!(
        "{:^20}  ", // two trailing spaces
        if print_year {
            format!("{month_name} {year}")
        } else {
            month_name.to_string()
        }
    ));

    lines.push("Su Mo Tu We Th Fr Sa  ".to_string()); // two trailing spaces

    for week in days.chunks(7) {
        lines.push(format!(
            "{:width$}  ", // two trailing spaces
            week.join(" "),
            width = LINE_WIDTH - 2
        ));
    }

    while lines.len() < 8 {
        lines.push(" ".repeat(LINE_WIDTH));
    }

    lines
}

// --------------------------------------------------
// #[cfg(test)]
// mod tests {
//     use super::{format_month, last_day_in_month, parse_month};
//     use chrono::NaiveDate;

//     #[test]
//     fn test_parse_month() {
//         let res = parse_month("1".to_string());
//         assert!(res.is_ok());
//         assert_eq!(res.unwrap(), 1u32);

//         let res = parse_month("12".to_string());
//         assert!(res.is_ok());
//         assert_eq!(res.unwrap(), 12u32);

//         let res = parse_month("jan".to_string());
//         assert!(res.is_ok());
//         assert_eq!(res.unwrap(), 1u32);

//         let res = parse_month("0".to_string());
//         assert!(res.is_err());
//         assert_eq!(
//             res.unwrap_err().to_string(),
//             r#"month "0" not in the range 1 through 12"#
//         );

//         let res = parse_month("13".to_string());
//         assert!(res.is_err());
//         assert_eq!(
//             res.unwrap_err().to_string(),
//             r#"month "13" not in the range 1 through 12"#
//         );

//         let res = parse_month("foo".to_string());
//         assert!(res.is_err());
//         assert_eq!(res.unwrap_err().to_string(), r#"Invalid month "foo""#);
//     }

//     #[test]
//     fn test_format_month() {
//         let today = NaiveDate::from_ymd_opt(0, 1, 1).unwrap();
//         let leap_february = vec![
//             "   February 2020      ",
//             "Su Mo Tu We Th Fr Sa  ",
//             "                   1  ",
//             " 2  3  4  5  6  7  8  ",
//             " 9 10 11 12 13 14 15  ",
//             "16 17 18 19 20 21 22  ",
//             "23 24 25 26 27 28 29  ",
//             "                      ",
//         ];
//         assert_eq!(format_month(2020, 2, true, today), leap_february);

//         let may = vec![
//             "        May           ",
//             "Su Mo Tu We Th Fr Sa  ",
//             "                1  2  ",
//             " 3  4  5  6  7  8  9  ",
//             "10 11 12 13 14 15 16  ",
//             "17 18 19 20 21 22 23  ",
//             "24 25 26 27 28 29 30  ",
//             "31                    ",
//         ];
//         assert_eq!(format_month(2020, 5, false, today), may);

//         let april_hl = vec![
//             "     April 2021       ",
//             "Su Mo Tu We Th Fr Sa  ",
//             "             1  2  3  ",
//             " 4  5  6 \u{1b}[7m 7\u{1b}[0m  8  9 10  ",
//             "11 12 13 14 15 16 17  ",
//             "18 19 20 21 22 23 24  ",
//             "25 26 27 28 29 30     ",
//             "                      ",
//         ];
//         let today = NaiveDate::from_ymd_opt(2021, 4, 7).unwrap();
//         assert_eq!(format_month(2021, 4, true, today), april_hl);
//     }

//     #[test]
//     fn test_last_day_in_month() {
//         assert_eq!(
//             last_day_in_month(2020, 1),
//             NaiveDate::from_ymd_opt(2020, 1, 31).unwrap()
//         );
//         assert_eq!(
//             last_day_in_month(2020, 2),
//             NaiveDate::from_ymd_opt(2020, 2, 29).unwrap()
//         );
//         assert_eq!(
//             last_day_in_month(2020, 4),
//             NaiveDate::from_ymd_opt(2020, 4, 30).unwrap()
//         );
//     }
// }
