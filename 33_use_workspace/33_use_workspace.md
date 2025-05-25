# 工作空间

工作空间相关学习笔记。

## 学习总结

1. 工作空间（Workspace）是一组共享同一个Cargo.lock和输出目录的包
2. 工作空间有助于组织大型项目，将相关功能分成多个包
3. 工作空间中的所有crate共享相同版本的依赖项
4. 工作空间通过根目录的Cargo.toml配置，使用`[workspace]`部分
5. 工作空间内的crate可以互相依赖
6. 使用`cargo build -p crate_name`编译工作空间中的特定crate
7. 使用`cargo test -p crate_name`测试工作空间中的特定crate
8. 使用`cargo run -p crate_name`运行工作空间中的特定二进制crate
9. 工作空间中的crate可以单独发布到crates.io
10. 每个工作空间成员有自己的src目录和Cargo.toml

## 工作空间结构示例

```
workspace/
├── Cargo.toml           // 工作空间配置
├── Cargo.lock           // 所有crate共享的锁文件
├── target/              // 共享的编译输出目录
├── crate1/              // 第一个crate
│   ├── Cargo.toml       // crate1配置
│   └── src/
│       └── lib.rs       // crate1代码
├── crate2/              // 第二个crate
│   ├── Cargo.toml       // crate2配置
│   └── src/
│       └── lib.rs       // crate2代码
└── app/                 // 主应用crate
    ├── Cargo.toml       // app配置
    └── src/
        └── main.rs      // 应用入口点
```

## 工作空间Cargo.toml示例

```toml
# 工作空间根目录的Cargo.toml
[workspace]
members = [
    "crate1",
    "crate2",
    "app",
]

# crate1/Cargo.toml
[package]
name = "crate1"
version = "0.1.0"
edition = "2021"

# crate2/Cargo.toml
[package]
name = "crate2"
version = "0.1.0"
edition = "2021"

[dependencies]
crate1 = { path = "../crate1" }

# app/Cargo.toml
[package]
name = "app"
version = "0.1.0"
edition = "2021"

[dependencies]
crate1 = { path = "../crate1" }
crate2 = { path = "../crate2" }
```

## 工作空间代码示例

```rust
// crate1/src/lib.rs
pub fn add_one(x: i32) -> i32 {
    x + 1
}

// crate2/src/lib.rs
use crate1::add_one;

pub fn add_two(x: i32) -> i32 {
    add_one(x) + 1
}

// app/src/lib
use crate1::add_one;
use crate2::add_two;

fn main() {
    let num = 10;
    println!("{} + 1 = {}", num, add_one(num));
    println!("{} + 2 = {}", num, add_two(num));
}
```

---

| [上一页：发布Crate](../32_publish_crate/32_publish_crate.md) | [下一页：智能指针Box](../34_use_Box/34_use_Box.md) |
|------------------------|------------------------| 