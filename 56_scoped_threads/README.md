# 作用域线程 Scoped Threads
- 定义：使用`std::thread::scope`创建的线程，生命周期受限与特定作用域
- 特性：线程在作用域结束前必须终止，无需手动管理joinHandle
# 解释代码
```rust
    let a = String::from("hello");
    thread::scope(|s| {
        let b = String::from("hello");
        for i in 0..5 {
            s.spawn(|| {
                thread::sleep(Duration::from_secs(1));
                println!("thread #{}", b);
            });
        }
    })
```
a可以打印，因为a的生命周期长，可以引用
b不行，因为b的生命周期比线程短，不可以引用
因此，要使用b就要转移所有权。
