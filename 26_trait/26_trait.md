# Rust 特征（Trait）

特征（Trait）是 Rust 中定义共享行为的一种方式。它类似于其他语言中的接口（Interface），但功能更强大。

## 基本概念

1. **特征定义**
   - 使用 `trait` 关键字定义
   - 可以包含方法签名和默认实现
   - 方法可以是关联函数或实例方法

2. **特征实现**
   - 使用 `impl` 关键字为类型实现特征
   - 必须实现特征中定义的所有方法
   - 可以为外部类型实现外部特征（孤儿规则）

## 示例代码

```rust
// 定义特征
pub trait Summary {
    fn summarize(&self) -> String;
}

// 为类型实现特征
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
```

## 特征作为参数

1. **impl Trait 语法**
   ```rust
   fn notify(item: impl Summary) {
       println!("Breaking news! {}", item.summarize());
   }
   ```

2. **特征约束语法**
   ```rust
   fn notify<T: Summary>(item: T) {
       println!("Breaking news! {}", item.summarize());
   }
   ```

## 特征约束

1. **多重约束**
   ```rust
   fn notify(item: impl Summary + Display)
   ```

2. **where 子句**
   ```rust
   fn notify<T>(item: T)
   where
       T: Summary + Display
   ```

## 返回实现特征的类型

```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("rust_lang"),
        content: String::from("Hello, world!"),
        reply: false,
        retweet: false,
    }
}
```

## 使用特征修复代码

特征可以用于修复泛型代码中的类型约束问题，确保类型具有所需的行为。

## 特征对象

1. **动态分发**
   - 使用 `Box<dyn Trait>`
   - 运行时多态

2. **对象安全**
   - 特征必须是对象安全的
   - 不能包含泛型参数

## 关联类型

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

## 默认泛型类型参数和运算符重载

```rust
trait Add<RHS=Self> {
    type Output;
    fn add(self, other: RHS) -> Self::Output;
}
```

## 完全限定语法

```rust
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```

## 使用 supertrait 来要求 trait 中实现其他 trait

```rust
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        // ...
    }
}
```

## 使用 newtype 模式在外部类型上实现外部 trait

```rust
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
```

## 学习总结

1. **特性(Trait)是Rust中实现多态的主要方式**，允许不同类型共享相同的行为
2. **特性定义使用`trait`关键字**：`trait Summary { fn summarize(&self) -> String; }`
3. **为类型实现特性使用`impl Trait for Type`语法**：`impl Summary for NewsArticle { ... }`
4. **特性可以有默认实现**，实现该特性的类型可以选择使用或覆盖默认实现
5. **特性作为函数参数**可以接收任何实现了该特性的类型：`fn print_summary(item: impl Summary)`
6. **特性约束**使用泛型更加明确：`fn print_summary<T: Summary>(item: T)`
7. **多重特性约束**通过`+`组合：`fn notify<T: Summary + Display>(item: T)`
8. **`where`子句**可以简化复杂的特性约束：`fn notify<T>(item: T) where T: Summary + Display`
9. **返回实现特性的类型**：`fn returns_summarizable() -> impl Summary`
10. **特性对象**允许动态分发：`Box<dyn Summary>`或`&dyn Summary`
11. **dyn Trait**用于运行时确定具体类型，实现运行时多态
12. **孤儿规则**：只能为当前crate中的类型实现当前crate中的特性
13. **关联类型**在特性中定义占位符类型：`trait Iterator { type Item; }`
14. **运算符重载**通过实现标准库中的特性实现：`impl Add for Point { ... }`
15. **特性边界**可以限制泛型类型必须实现特定特性
16. **supertrait**要求实现某特性必须先实现另一特性：`trait Printable: Display { ... }`
17. **特性可以作为标记**，表明类型具有某种属性或行为
18. **newtype模式**可以绕过孤儿规则，为外部类型实现外部特性
19. **特性是类型安全的**，编译器会在编译时检查类型是否正确实现了所需特性
20. **特性是Rust类型系统的核心部分**，广泛用于标准库和第三方库中

---

| [上一页：泛型](../25_generic/25_generic.md) | 下一页 |
|------------------------|------------|