# RefCell_38
# 目录
- [RefCell_38](#refcell_38)
- [学习总结](#学习总结)
- [示例代码](#示例代码)

`RefCell` 提供了内部可变性，允许在不可变引用的情况下修改数据。

## 学习总结

1. `RefCell` 是 Rust 的智能指针，提供运行时的可变性检查。
2. 使用 `RefCell::new(value)` 创建一个包含初始值的 `RefCell`。
3. 使用 `borrow()` 获取不可变引用，使用 `borrow_mut()` 获取可变引用。
4. `RefCell` 的借用规则：
    - 同一时间只能有一个可变借用，或多个不可变借用。
    - 违反规则会在运行时引发 `panic`。
5. 常见用法：
    - 在单线程场景中提供内部可变性。
    - 与 `Rc` 结合使用，实现共享且可变的数据。

# 示例代码：

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);

    // 修改数据
    *data.borrow_mut() += 1;

    // 读取数据
    println!("数据: {}", data.borrow());
}
```
6. 注意事项：
    - `RefCell` 适用于单线程场景，不能在多线程中使用。
    - 使用 `RefCell` 时要小心借用规则，避免运行时错误。

| [上一页：Rc](../37_Rc/37_Rc.md) | | 