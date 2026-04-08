# rust_learn


> base command
```bash
cargo new project

#[dependencies]
cargo add clap
#[dev-dependencies]
cargo add --dev assert_cmd predicates
cargo add  clap --features derive

rustup update stable    # 更新 rust 工具链
cargo --version  # 查看 cargo 版本
rustc --version  # 查看 rustc 版本

cargo test  // 测试所有代码中的#[cfg(test)]
cargo test --bin <name> // 只运行指定二进制文件的测试, bin 目录
cargo test --lib // 只运行 src/lib.rs 中的单元测试
cargo test <pattern> // 运行名称匹配模式的测试函数（支持模糊匹配）
cargo test function // function为测试#[cfg(test)]作用域内的函数名
cargo test --test <filename> // 测试tests 目录下的文件
cargo test --test cli runs // src/tests/cli.rs  // runs()

cargo run -- --name=Alice -v  # 参数
cargo run -- arg1 arg2 arg3
cargo run --quiet --bin true # src/bin/true.rs

#Cargo.toml 中 name = "hello"
cargo run hello # 运行主程序
```

# 国内镜像/源
>.bashrc
```bash
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
```
>$HOME/.cargo/config.toml
```
[source.crates-io]
replace-with = 'ustc'

[source.rsproxy-sparse]
registry = "sparse+https://rsproxy.cn/index/"

[source.rsproxy]
registry = "https://rsproxy.cn/crates.io-index"

[source.ustc]
registry = "sparse+https://mirrors.ustc.edu.cn/crates.io-index/"

[registries.ustc]
index = "sparse+https://mirrors.ustc.edu.cn/crates.io-index/"

[net]
git-fetch-with-cli = true  # 避免部分环境下 libgit2 超时
```
