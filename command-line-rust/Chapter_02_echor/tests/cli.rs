// use assert_cmd::Command;
// use predicates::prelude::*;


// #[test]
// fn dies_no_args() {
//     let mut cmd = Command::cargo_bin("echor").unwrap();
//     cmd.assert()
//         .failure() // 断言命令执行失败(返回非零退出码)
//         .stderr(predicates::str::contains("USAGE")); // 断言标准错误输出满足特定条件
//         // 使用谓词断言 stderr 包含 "USAGE" 字符串
// }

#[test] //过程宏(procedural macro),标记下面的函数是一个测试函数
fn runs() {
    let mut cmd = Command::cargo_bin("echor") // Result<Command, Error>
                    .unwrap(); // 提取Option<Some(T),None> / Result<Ok(T),Err(E)>类型值的快捷方式,但在错误情况下会触发panic
    cmd.arg("hello")
        .assert() // 执行配置的命令并返回一个 Assert 对象
        .success(); // 断言命令执行成功(退出码为 0)
}

// use std::fs;
// #[test] 
// fn hello1() {
//     let outfile = "tests/expected/hello1.txt";
//     let expected = fs::read_to_string(outfile).unwrap(); // 读取文件到字符串
//     let mut cmd = Command::cargo_bin("echor").unwrap();
//     cmd.arg("Hello there") // 提供参数
//         .assert()
//         .success()
//         .stdout(expected);
// }

use assert_cmd::Command;
// use predicates::prelude::*;
use std::fs;

type TestResult = Result<(),Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?; // 使用? 而不是 Result::unwrap 来解包Ok值或者传播Err
    cmd.assert()
        .failure() // 断言命令执行失败(返回非零退出码)
        .stderr(predicates::str::contains("USAGE")); // 断言标准错误输出满足特定条件
        // 使用谓词断言 stderr 包含 "USAGE" 字符串\
    Ok(())
}


#[test] 
fn hello1() -> TestResult {
    let outfile = "tests/expected/hello1.txt";
    let expected = fs::read_to_string(outfile)?; // 读取文件到字符串
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("Hello there") // 提供参数
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test] 
fn hello2() -> TestResult {
    let expected = fs::read_to_string("tests/expected/hello2.txt")?; // 读取文件到字符串
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(vec!["Hello","there"]) // 使用args 传递一个参数向量
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}


fn run(args: &[&str],expected_file: &str) -> TestResult { // args  参切片 内容为字符串引用
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}




// // --------------------------------------------------
// #[test]
// fn hello1() -> Result<()> {
//     run(&["Hello there"], "tests/expected/hello1.txt")
// }

// // --------------------------------------------------
// #[test]
// fn hello2() -> Result<()> {
//     run(&["Hello", "there"], "tests/expected/hello2.txt")
// }

// // --------------------------------------------------
// #[test]
// fn hello1_no_newline() -> Result<()> {
//     run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
// }

// // --------------------------------------------------
// #[test]
// fn hello2_no_newline() -> Result<()> {
//     run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
// }