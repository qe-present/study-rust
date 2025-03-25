# Generic_25

泛型相关学习笔记。

## 学习总结

1. 泛型允许定义可适用于多种类型的代码
2. 函数泛型：`fn largest<T>(list: &[T]) -> &T { ... }`
3. 结构体泛型：`struct Point<T> { x: T, y: T }`
4. 可以使用多个泛型参数：`struct Point<T, U> { x: T, y: U }`
5. 枚举泛型：`enum Option<T> { Some(T), None }`和`enum Result<T, E> { Ok(T), Err(E) }`
6. 方法定义中的泛型：`impl<T> Point<T> { fn x(&self) -> &T { &self.x } }`
7. 可以为特定类型实现特定方法：`impl Point<f32> { ... }`
8. 特性（Traits）定义共享行为：`trait Summary { fn summarize(&self) -> String; }`
9. 实现特性：`impl Summary for NewsArticle { ... }`
10. 特性作为参数：`fn notify(item: &impl Summary) { ... }`
11. 特性约束：`fn notify<T: Summary>(item: &T) { ... }`
12. 多重特性约束：`fn notify<T: Summary + Display>(item: &T) { ... }`
13. `where`子句简化复杂约束：`fn notify<T>(item: &T) where T: Summary + Display { ... }`
14. 返回实现特性的类型：`fn returns_summarizable() -> impl Summary { ... }`
15. 使用特性约束有条件地实现方法：`impl<T: Display + PartialOrd> Pair<T> { ... }`
16. 生命周期泛型确保引用有效性：`fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { ... }` 