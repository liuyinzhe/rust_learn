
#### 创建虚拟环境
python -m venv .venv
#### 激活环境，安装maturin
source .venv/bin/activate
pip install maturin
#### 编辑预期的main.py

#### 初始化，生成例子
maturin init --bindings pyo3
>src/lib.rs 基础上修改
#### 编译,可选的写pyi 文件
# 开发编译安装,就可以测试用python调用了
maturin develop
>生成在.venv/lib/maturin_test/maturin_test.cpython-311-x86_64-linux-gnu.so
# release版本编译,只会编译打包,不会更新安装到python环境
maturin build --release

#### so 与main.py 可单独使用
