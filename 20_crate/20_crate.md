# Crate_20

Rust包和模块相关学习笔记。

## 学习总结

1. crate是Rust编译的最小代码单位
2. crate可以是二进制crate或库crate
3. 包（package）是一个或多个crate的集合，由`Cargo.toml`文件定义
4. 模块（module）使用`mod`关键字定义，用于组织代码和控制可见性
5. 路径（path）用于引用模块树中的项，可以是绝对路径或相对路径
6. 使用`pub`关键字使项公开可见
7. `use`关键字将路径引入作用域，简化代码引用
8. 使用嵌套路径简化多个`use`语句：`use std::{cmp::Ordering, io};`
9. 通配符`*`引入所有公共项：`use std::collections::*;`
10. `pub use`重新导出，使导入的项对外部代码可用
11. 使用外部包需要在`Cargo.toml`中添加依赖
12. 模块可以使用单独的文件或目录组织 