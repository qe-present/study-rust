# LazyCell 与 LazyLock 示例

## 项目概述

本项目演示了 Rust 中 `LazyCell` 和 `LazyLock` 的用法，这两个类型用于延迟初始化（lazy initialization）。

## 技术栈

- **语言**: Rust 2024 Edition
- **依赖**: `crossbeam` (版本 0.8)

## 核心概念

### LazyCell (单线程)
- `std::cell::LazyCell`: 用于单线程环境的延迟初始化
- 在首次访问时才执行初始化函数
- 示例演示了延迟初始化的行为：`println!` 只执行一次

### LazyLock (多线程)
- `std::sync::LazyLock`: 用于多线程安全的延迟初始化
- 适合用于静态变量的初始化
- 保证初始化函数只被执行一次，即使有多个线程同时访问

## 测试示例

### 1. 单线程 LazyCell 测试 (`lazy_cell_works`)
- 演示 `LazyCell::new()` 创建延迟初始化值
- 验证初始化函数只在第一次解引用时执行
- 输出显示初始化代码只执行一次

### 2. 多线程 LazyLock 测试 (`lazy_lock_works`)
- 演示 `LazyLock` 在 5 个线程中的使用
- 验证多个线程同时访问时，初始化只执行一次
- 使用 `crossbeam::scope` 进行线程管理

## 运行测试

```bash
cargo test
```

## 学习要点

1. **延迟初始化**: 将高开销的操作推迟到真正需要时才执行
2. **线程安全**: `LazyLock` 确保在多线程环境下的安全初始化
3. **性能优化**: 避免不必要的计算，特别是对于静态数据
4. **跨beam 库**: 使用 `crossbeam` 进行高效的线程操作

## 关键源码位置

- 主测试代码: `src/lib.rs:1-34`
