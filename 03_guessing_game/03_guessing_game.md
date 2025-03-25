# Guessing_game_3

猜数字游戏。

## 学习总结

1. 使用`use std::io`导入标准库的输入/输出功能
2. `String::new()`创建新的空字符串
3. `io::stdin().read_line(&mut guess)`读取用户输入
4. 使用`&mut`表示可变引用
5. `Result`类型表示操作可能成功或失败
6. `.expect()`处理潜在错误
7. 使用外部依赖（如`rand`）需要在`Cargo.toml`中添加
8. `match`表达式用于流程控制，类似于增强版的switch语句
9. 类型转换：`let guess: u32 = guess.trim().parse().expect("请输入数字！");`
10. `loop`创建无限循环，`break`用于退出循环
11. 使用`Ordering`枚举比较数值大小