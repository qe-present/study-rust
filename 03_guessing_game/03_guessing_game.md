# Guessing_game_3

猜数游戏：学习Rust的基本概念，如变量、函数、循环等。

## 学习总结

1. 使用`let`声明变量
2. 默认情况下，Rust变量是不可变的（immutable）
3. 使用`mut`关键字使变量可变
4. `String::new()`创建新的空字符串
5. `&`表示引用（reference）
6. `stdin()`函数返回标准输入的句柄
7. `read_line()`方法从标准输入读取一行文本
8. `&mut guess`表示可变引用
9. `Result`类型表示操作可能成功或失败
10. `expect()`方法处理`Result`类型的错误
11. `println!`宏支持字符串插值：`{}`是占位符
12. `Rng` trait提供随机数功能
13. `rand::thread_rng()`返回随机数生成器
14. `gen_range()`生成指定范围内的随机数
15. `parse()`方法将字符串转换为数字
16. `trim()`方法去除字符串首尾的空白字符
17. `match`表达式处理多种可能的结果
18. `loop`创建无限循环
19. `break`语句退出循环
20. `continue`语句跳转到下一次循环迭代
21. `Ordering`枚举有三个变体：`Less`、`Greater`和`Equal`
22. 外部依赖在`Cargo.toml`的`[dependencies]`部分指定

---

| [上一页：Hello Cargo](../02_hello_cargo/02_hello_cargo.md) | [下一页：变量](../04_variables/04_variables.md) |
|------------------------|------------------------|