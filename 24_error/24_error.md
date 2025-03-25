# Error_24

错误处理相关学习笔记。

## 学习总结

1. Rust将错误分为两大类：可恢复错误和不可恢复错误
2. 可恢复错误使用`Result<T, E>`枚举处理
3. 不可恢复错误使用`panic!`宏处理，程序会终止执行
4. 当`panic!`发生时，程序默认会展开栈，清理数据
5. 可以通过`Cargo.toml`配置让程序直接终止，减小二进制文件大小
6. `Result<T, E>`定义：`enum Result<T, E> { Ok(T), Err(E) }`
7. 处理`Result`的方法：`match`、`unwrap`、`expect`、`?`运算符等
8. `unwrap`在`Err`时会自动调用`panic!`
9. `expect`允许自定义`panic!`信息
10. `?`运算符简化错误传播：`let f = File::open("hello.txt")?;`
11. `?`运算符只能用于返回`Result`或`Option`类型的函数
12. 可以自定义错误类型并实现`std::error::Error`特性
13. 通常使用`thiserror`或`anyhow`等crate简化错误处理 