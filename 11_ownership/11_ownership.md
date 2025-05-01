# Ownership_11

所有权系统相关学习笔记。

## 学习总结

1. 引用（References）允许访问值而不获取所有权
2. 借用（Borrowing）是创建引用的过程
3. 不可变引用（Immutable References）：`&T`
4. 可变引用（Mutable References）：`&mut T`
5. 可变引用的限制：同一时间只能有一个
6. 不能同时拥有可变引用和不可变引用
7. 引用的作用域与悬垂引用（Dangling References）
8. 引用规则：
   - 任意时刻，只能有一个可变引用或任意数量的不可变引用
   - 引用必须始终有效（编译器确保）
9. 借用检查器（Borrow Checker）在编译时执行这些规则 

---

| [上一页：所有权(10)](../10_ownership/10_ownership.md) | [下一页：所有权(12)](../12_ownership/12_ownership.md) |
|------------------------|------------------------| 