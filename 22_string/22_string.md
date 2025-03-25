# String_22

字符串相关学习笔记。

## 学习总结

1. Rust核心语言中只有字符串切片`str`，而`String`类型来自标准库
2. `String`是可增长、可变、有所有权的UTF-8编码字符串
3. 创建新字符串：`let mut s = String::new();`或`let s = "initial".to_string();`
4. 从字符串字面量创建：`let s = String::from("hello");`
5. 更新字符串：使用`push_str()`方法添加字符串切片
6. 添加单个字符：`push()`方法
7. 字符串拼接：使用`+`运算符或`format!`宏
8. `+`运算符使用`fn add(self, s: &str) -> String`，所以会移动第一个字符串
9. `format!`宏更灵活且不获取所有权：`format!("{}-{}", s1, s2)`
10. Rust字符串不支持索引访问：`s[0]`无效
11. 字符串是UTF-8编码，一个字符可能占用多个字节
12. 可以通过`chars()`方法遍历字符（Unicode标量值）
13. 可以通过`bytes()`方法遍历原始字节 