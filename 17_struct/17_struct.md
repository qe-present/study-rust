# Struct_17

结构体相关学习笔记。

## 学习总结

1. 结构体是自定义数据类型，可组合多个相关联的值
2. 使用`struct`关键字定义结构体：`struct User { username: String, ... }`
3. 创建结构体实例：`let user1 = User { username: String::from("username"), ... }`
4. 访问结构体字段：`user1.username`
5. 修改可变结构体的字段：`user1.username = String::from("newname")`
6. 字段初始化简写：当变量名与字段名相同时，可简写
7. 结构体更新语法：`User { email: new_email, ..user1 }`
8. 元组结构体：有名称但字段无名的结构体
9. 类单元结构体：没有任何字段的结构体
10. 结构体数据所有权：结构体实例拥有其所有数据的所有权
11. 结构体方法使用`impl`块定义
12. 方法第一个参数通常是`&self`，表示调用该方法的结构体实例
13. 结构体关联函数（如`String::from`）没有`self`参数 