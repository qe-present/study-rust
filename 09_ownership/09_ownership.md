# Ownership_9

所有权系统相关学习笔记。

## 学习总结

1. 引用（References）允许使用值但不获取其所有权
2. 创建引用的行为称为借用（Borrowing）
3. 不可变引用：`&T`，可以有多个
4. 可变引用：`&mut T`，同一时间只能有一个
5. 不能同时拥有不可变引用和可变引用
6. 引用的作用域从声明点开始，到最后一次使用结束
7. 引用必须始终有效，编译器防止悬垂引用（Dangling References）
8. 引用规则确保了内存安全和防止数据竞争 

---

| [上一页：所有权(8)](../08_ownership/08_ownership.md) | [下一页：所有权(10)](../10_ownership/10_ownership.md) |
|------------------------|------------------------| 