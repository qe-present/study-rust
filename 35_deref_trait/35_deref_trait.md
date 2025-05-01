# Deref特性

Deref特性相关学习笔记。

## 学习总结

1. `Deref`特性允许自定义解引用运算符（`*`）的行为
2. 通过实现`Deref`，可以使智能指针像常规引用一样工作
3. `Deref`特性的定义：
   ```rust
   pub trait Deref {
       type Target: ?Sized;
       fn deref(&self) -> &Self::Target;
   }
   ```
4. 实现`Deref`特性需要定义关联类型`Target`和`deref`方法
5. 当使用`*`运算符时，Rust实际上调用`*`运算符背后的`deref`方法，然后再进行解引用
6. Rust提供了`Deref`强制转换（deref coercion）功能，自动调用多次`deref`
7. `Deref`强制转换优化：
   - 当`T: Deref<Target=U>`，允许`&T`隐式转换为`&U`
   - 当`T: DerefMut<Target=U>`，允许`&mut T`隐式转换为`&mut U`
   - 当`T: Deref<Target=U>`，允许`&mut T`隐式转换为`&U`
8. `DerefMut`特性用于可变引用的解引用
9. `Deref`强制转换发生在编译时，没有运行时开销
10. 标准库的智能指针（如`Box<T>`、`Rc<T>`、`String`）都实现了`Deref`特性

## 使用示例

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // 使用解引用运算符
    
    // 等价于：
    // assert_eq!(5, *(y.deref()));
}
```

## Deref强制转换示例

```rust
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    
    // Deref强制转换：&MyBox<String> -> &String -> &str
    hello(&m);
    
    // 不使用Deref强制转换的等价写法：
    hello(&(*m)[..]);
}
```

---

| [上一页：智能指针Box](../34_use_Box/34_use_Box.md) | [下一页：Drop特性](../36_drop_trait/36_drop_trait.md) |
|------------------------|------------------------| 