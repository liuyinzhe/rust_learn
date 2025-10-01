use assert_cmd::Command; // cargo add assert_cmd
use pretty_assertions::assert_eq; // cargo add pretty_assertions

#[test] // 告知测试以下函数
fn runs() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    // cmd.assert().success().stdout("Hello,world\n");
    let output = cmd.output().expect("fail");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, "Hello, world!\n");
}

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_not_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}


// #[test]
// fn runs_old() {
//     use std::process::Command;
//     let mut cmd = Command::new("ls");
//     let res = cmd.output();
//     assert!(res.is_ok());
// }

