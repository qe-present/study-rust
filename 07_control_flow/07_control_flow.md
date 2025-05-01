# Control_flow_7

控制流相关学习笔记。

## 学习总结

1. `if`表达式：`if condition { ... } else { ... }`
2. 条件必须是`bool`类型，Rust不会自动转换非布尔类型
3. 可以使用`else if`处理多个条件
4. `if`是表达式，可以用于赋值：`let number = if condition { 5 } else { 6 };`
5. 所有可能的`if`分支返回值类型必须相同
6. 循环类型：`loop`、`while`和`for`
7. `loop`创建无限循环，需要使用`break`退出
8. 可以从`loop`返回值：`let result = loop { break value; };`
9. `while`循环：`while condition { ... }`
10. `for`循环遍历集合：`for element in collection { ... }`
11. `for`循环使用`Range`：`for number in (1..4) { ... }`
12. `continue`语句跳过当前循环迭代的剩余部分
13. 循环标签用于在嵌套循环中指定`break`或`continue`的目标 

---

| [上一页：函数](../06_functions/06_functions.md) | [下一页：所有权(8)](../08_ownership/08_ownership.md) |
|------------------------|------------------------| 