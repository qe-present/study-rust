# 智能指针Box

Box智能指针相关学习笔记。

## 学习总结

1. `Box<T>`是最简单的智能指针，允许将数据存储在堆上而不是栈上
2. 使用`Box::new(value)`将值放在堆上
3. `Box<T>`实现了`Deref`特性，可以像引用一样使用
4. `Box<T>`实现了`Drop`特性，离开作用域时会自动释放堆内存
5. `Box<T>`的常见用途：
   - 编译时无法确定大小的类型（如递归类型）
   - 需要转移大型数据所有权而不复制数据
   - 使用特性对象实现多态
6. `Box<T>`是一个只在栈上占用固定大小空间的指针，指向堆上的数据
7. 可以使用`*`操作符解引用`Box<T>`来访问存储的值
8. Rust的自动解引用功能使得大多数情况下不需要显式解引用
9. `Box<T>`有助于解决递归数据结构问题，例如链表、树等
10. `Box<T>`没有额外的性能开销，但也没有额外的功能（如引用计数）

## 使用示例

```rust
fn main() {
    // 创建一个Box，保存一个整数
    let b = Box::new(5);
    println!("b = {}", b); // 自动解引用
    
    // 使用解引用操作符访问值
    let value = *b;
    println!("value = {}", value);
}
```

## 递归类型示例

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    
    // 使用模式匹配访问列表
    match list {
        Cons(head, tail) => {
            println!("Head value: {}", head);
            // ...处理tail
        },
        Nil => println!("Empty list"),
    }
}
```

## 特性对象示例

```rust
trait Draw {
    fn draw(&self);
}

struct Button {
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing a button with label: {}", self.label);
    }
}

struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button { label: String::from("OK") }),
            // 可以添加其他实现Draw特性的类型...
        ],
    };
    
    screen.run();
}
```

---

| [上一页：工作空间](../33_use_workspace/33_use_workspace.md) | [下一页：Deref特性](../35_deref_trait/35_deref_trait.md) |
|------------------------|------------------------| 