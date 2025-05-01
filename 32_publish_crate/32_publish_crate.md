# 发布Crate

发布Crate相关学习笔记。

## 学习总结

1. crates.io是Rust官方的包注册中心，可以发布和下载开源库
2. 发布前需要注册crates.io账号并获取API token
3. 使用`cargo login <token>`存储API token
4. 在Cargo.toml中添加必要的元数据：
   - name: crate名称
   - version: 版本号（遵循语义化版本）
   - authors: 作者信息
   - description: 简短描述
   - license: 许可证
   - repository: 源代码仓库
5. 使用`cargo publish`命令发布crate
6. 使用`cargo yank --vers <version>`撤回特定版本（不会删除代码）
7. 撤回操作可以通过`--undo`撤销
8. 发布后的crate不能删除，需谨慎考虑发布内容
9. 可以使用工作空间组织多个相关crate
10. 使用良好的文档和测试提高crate质量
11. 使用`#[doc]`属性和文档注释为函数和类型添加文档
12. 遵循语义化版本控制（SemVer）原则更新crate版本

## Cargo.toml 示例

```toml
[package]
name = "my_crate"
version = "0.1.0"
authors = ["Your Name <your.email@example.com>"]
edition = "2021"
description = "A short description of the crate"
license = "MIT OR Apache-2.0"
repository = "https://github.com/yourusername/my_crate"
keywords = ["keyword1", "keyword2"]
categories = ["category1", "category2"]
readme = "README.md"

[dependencies]
# 依赖项...
```

## 文档注释示例

```rust
/// 将两个数字相加
///
/// # Examples
///
/// ```
/// let sum = my_crate::add(1, 2);
/// assert_eq!(sum, 3);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 一个表示矩形的结构体
///
/// # Fields
///
/// * `width` - 矩形的宽度
/// * `height` - 矩形的高度
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    /// 创建一个新的矩形
    ///
    /// # Examples
    ///
    /// ```
    /// let rect = my_crate::Rectangle::new(10, 20);
    /// assert_eq!(rect.width, 10);
    /// assert_eq!(rect.height, 20);
    /// ```
    pub fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}
```

## 发布命令示例

```bash
# 获取API token
cargo login abcdefghijklmnopqrstuvwxyz012345

# 检查发布准备情况
cargo publish --dry-run

# 正式发布
cargo publish

# 撤回特定版本
cargo yank --vers 0.1.0

# 撤销撤回操作
cargo yank --vers 0.1.0 --undo
```

---

| [上一页：循环与迭代器](../31_loop_vs_iterator/31_loop_vs_iterator.md) | [下一页：工作空间](../33_use_workspace/33_use_workspace.md) |
|------------------------|------------------------| 