# 循环与迭代器

循环与迭代器对比相关学习笔记。

## 学习总结

1. 迭代器模式允许对一系列元素执行相同的操作
2. 迭代器在Rust中是惰性的，不使用不会消耗
3. 迭代器都实现了`Iterator` trait，其核心方法是`next()`
4. `next()`方法返回`Option<Item>`，要么是`Some(value)`，要么是`None`
5. 可以使用`iter()`、`into_iter()`和`iter_mut()`创建迭代器
   - `iter()` 返回不可变引用的迭代器
   - `into_iter()` 返回拥有所有权的迭代器
   - `iter_mut()` 返回可变引用的迭代器
6. 常用迭代器方法：
   - `map()`: 对每个元素应用函数
   - `filter()`: 保留满足条件的元素
   - `collect()`: 将迭代器转换为集合
   - `fold()`: 累积操作，如求和
   - `zip()`: 将两个迭代器组合
7. 迭代器是零成本抽象，编译后性能与手写循环相当或更优
8. 迭代器适合函数式编程风格，代码更简洁可读
9. 迭代器可以连锁调用多个操作，形成处理流水线
10. 自定义类型可以通过实现`Iterator` trait成为迭代器

## 循环实现与迭代器对比

```rust
// 使用循环实现求和
fn sum_with_loop(v: &[i32]) -> i32 {
    let mut sum = 0;
    for i in v {
        sum += i;
    }
    sum
}

// 使用迭代器实现求和
fn sum_with_iterator(v: &[i32]) -> i32 {
    v.iter().sum()
}

// 使用循环实现筛选和转换
fn even_squares_with_loop(v: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    for i in v {
        if i % 2 == 0 {
            result.push(i * i);
        }
    }
    result
}

// 使用迭代器实现筛选和转换
fn even_squares_with_iterator(v: &[i32]) -> Vec<i32> {
    v.iter()
     .filter(|&x| x % 2 == 0)
     .map(|&x| x * x)
     .collect()
}
```

## 自定义迭代器示例

```rust
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// 使用自定义迭代器
fn main() {
    let sum: u32 = Counter::new(5)
        .filter(|&x| x % 2 == 0)
        .map(|x| x * x)
        .sum();
    println!("Sum of even squares in Counter: {}", sum); // 4^2 + 2^2 = 20
}
```

---

| [上一页：命令行程序](../30_minigrep/30_minigrep.md) | [下一页：发布Crate](../32_publish_crate/32_publish_crate.md) |
|------------------------|------------------------|