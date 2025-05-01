# Drop特性

Drop特性相关学习笔记。

## 学习总结

1. `Drop`是Rust标准库中定义的特性，用于自定义类型被丢弃时的行为
2. 当值离开作用域时，`drop`方法会被自动调用
3. `Drop`特性的定义：
   ```rust
   pub trait Drop {
       fn drop(&mut self);
   }
   ```
4. 不能手动调用`drop`方法，否则会导致"双重释放"错误
5. 如果需要提前清理变量，可以使用`std::mem::drop`函数：`drop(value)`
6. `Drop`特性在实现资源管理（如文件句柄、网络连接等）时非常有用
7. `Drop`的调用顺序与创建顺序相反，遵循后进先出（LIFO）原则
8. 结构体中的字段也会自动调用它们的`drop`方法
9. 智能指针如`Box<T>`、`Rc<T>`、`RefCell<T>`等都实现了`Drop`特性
10. `Drop`实现是Rust实现RAII（资源获取即初始化）模式的核心

## 使用示例

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    // c和d离开作用域时，会调用它们的drop方法
}
```

## 提前释放示例

```rust
fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    // 提前释放c
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
```

---

| [上一页：Deref特性](../35_deref_trait/35_deref_trait.md) | [下一页：引用计数](../37_Rc/37_Rc.md) |
|------------------------|------------------------| 