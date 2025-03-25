# Variables_4

变量相关学习笔记。

## 学习总结

1. 使用`let`关键字声明变量
2. Rust变量默认是不可变的（immutable）
3. 使用`mut`关键字声明可变变量：`let mut x = 5;`
4. 常量使用`const`关键字声明，必须指定类型：`const MAX_POINTS: u32 = 100_000;`
5. 常量在整个程序运行期间都有效，且只能设置为常量表达式
6. 变量遮蔽（Shadowing）：使用相同名称的新变量声明会遮蔽之前的变量
7. 遮蔽与`mut`的区别：可以改变变量类型，且新变量仍然是不可变的
8. 遮蔽示例：`let spaces = "   "; let spaces = spaces.len();` 