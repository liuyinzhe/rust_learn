[TOC]

## 安装/卸载/更新

```bash
#安装(需要联网)
https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe

# 更新rustc cargo
rustup update
# version
rustc --version
cargo --version

# 卸载
rustup self uninstall
## 注意,需要搜索rustc/cargo删除干净；避免更新后无法找到
```



## 编译命令

### 创建二进制项目

```bash
cargo new --bin new_binary
```
### 创建动态链接项目

```bash
cargo new --lib mylib
```

## 依赖构建

### 添加依赖

```rust
cargo add time
```

cargo.toml

```toml
[dependencies]
time = "0.3.28"
```

### 更新依赖

```
cargo update
```



## Cargo配置

### 编译优化配置

Cargo.toml

```toml
[profile.release]
opt-level = "z"  # 最小体积优化（可选"s"平衡速度与体积）
lto = true       # 链接时优化（提升性能但增加编译时间）
codegen-units = 1# 限制并行代码生成单元数量（提升优化强度）
panic = "abort"  # 替换 panic 展开为直接终止（减少二进制体积）
strip = true     # 自动剥离调试符号（需 Rust 1.59+）
```

### 动态库构建配置

Cargo.toml

```toml
[lib]
name = "mylib"
crate-type = ["cdylib"]  # 生成 C 兼容动态库（推荐跨语言调用）
# crate-type = ["dylib"] # 生成原生 Rust 动态库（仅限 Rust 项目间共享）
```

编译：

发布模式

```bash
cargo build --relese
```

指定平台

```bash
# WebAssembly
cargo build --relese --target  wasm32-unknown-unknown

# Linux 平台
## x86_64 架构 x86_64-unknown-linux-gnu
## ARM 架构 aarch64-unknown-linux-gnu
#x86_64-unknown-linux-gnu
#x86_64-unknown-linux-gnux32
#x86_64-unknown-linux-musl
## 构建 Linux 平台的静态链接二进制（使用 MUSL）
cargo build --target x86_64-unknown-linux-musl

# Windows 平台
## MSVC 工具链 x86_64-pc-windows-msvc
cargo build --relese --target  x86_64-pc-windows-msvc
## GNU 工具链 x86_64-pc-windows-gnu
cargo build --relese --target  x86_64-unknown-linux-gnu
```



## 代码目录解构

项目代码目录风格

>binary crate 可执行，需要main函数
>library crate 没有main函数，无法执行；定义了一些功能可共享使用

入口点

>binary crate : src/main.rs
>library crate: src/lib.rs

src/lib.rs
```rust
pub mod log;
pub mod tools;
```

src/main.rs

```rust
use crate_name::Config; //cargo new crate_name
use crate_name::tools::model::function;
```

### 模块树

crate_name作为根路径，"::"作为路径链接，找到对应的model.rs以及其中的function函数

```python
src
	main.rs
	lib.rs
	Config.rs
    log.rs
	tools
    	mod.rs   # pub mod model;
		model.rs # fn function{()}
benches	##benchmakr
examples
tests
```

mod.rs 需要 pub 公有化

```rust
pub mod model
```

