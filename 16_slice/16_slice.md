# Slice_16

切片类型相关学习笔记。

## 学习总结

1. 切片是对集合中部分连续元素的引用
2. 切片不拥有所有权
3. 字符串切片语法：`&string[start_index..end_index]`
4. 如省略起始索引，则默认为0：`&string[..end_index]`
5. 如省略结束索引，则默认为长度：`&string[start_index..]`
6. 可以省略两个索引，获取整个字符串的切片：`&string[..]`
7. 字符串字面量是字符串切片：`let s: &str = "Hello, world!";`
8. 字符串切片类型写作`&str`
9. 函数接收字符串切片参数更灵活：`fn first_word(s: &str) -> &str {}`
10. 数组切片同样适用：`let a = [1, 2, 3, 4, 5]; let slice = &a[1..3];` 

---

| [上一页：所有权(15b)](../15b_ownership/15b_ownership.md) | [下一页：结构体](../17_struct/17_struct.md) |
|------------------------|------------------------| 