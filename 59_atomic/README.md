# Rust 原子类型（Atomic Types）详解

## 原子类型是什么

Rust 的原子类型提供了一种在多线程环境中**安全地共享和修改数据**的方式，避免了数据竞争。它们位于 `std::sync::atomic` 模块中。

### 基本特性

1. **原子操作不可分割**: 多个线程同时访问原子数据时，操作要么完全完成，要么完全不执行
2. **线程安全共享**: 允许在不同线程之间安全地共享和修改值
3. **内存顺序（Memory Ordering）**: 原子操作支持不同的内存顺序约束，控制操作的可见性顺序

```rust
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::thread;

// 原子计数器示例
fn main() {
    let counter = Arc::new(AtomicU32::new(0));

    // 创建10个线程,每个线程增加100次
    let mut handles = vec![];
    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 保证结果为1000,不会发生数据竞争
    println!("最终计数: {}", counter.load(Ordering::SeqCst));
}
```

### 可用的原子类型

Rust 提供以下原子类型:
- `AtomicBool`
- `AtomicI8`, `AtomicI16`, `AtomicI32`, `AtomicI64`, `AtomicIsize`
- `AtomicU8`, `AtomicU16`, `AtomicU32`, `AtomicU64`, `AtomicUsize`
- `AtomicPtr<T>`

### 内存顺序（Ordering）

原子操作可以指定内存顺序:
- `Ordering::Relaxed`: 最弱保证,Ordering::Relaxed 是 Rust 原子操作中最宽松的内存顺序（memory order）约束，只保证原子性，不保证操作顺序
- `Ordering::Acquire/Release`: 用于锁
- `Ordering::AcqRel`: 组合Acquire和Release
- `Ordering::SeqCst`: 最强保证,全局顺序一致性

### 项目代码示例

这个项目中的代码示例展示了几种典型的原子用法:
- `1_basic.rs`: 基础原子操作和内存顺序
- `2_advanced.rs`: 更复杂的原子用法
- `3_lockfree.rs`: 无锁队列的实现

# 其他记录
- cargo run --bin 2_advanced out+err> combined.log 重定向到文件
- 在Cargo.toml里面可以写多个bin，如下
```toml
[[bin]]
name = "main"
path = "src/main.rs"

[[bin]]
name = "2_advanced"
path = "src/2_advanced.rs"
```
- 输出到文件 cargo run --bin 3_lockfree out+err> combined.log
