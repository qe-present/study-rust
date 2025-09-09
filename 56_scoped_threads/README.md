# 作用域线程 Scoped Threads
- 定义：使用`std::thread::scope`创建的线程，生命周期受限与特定作用域
- 特性：线程在作用域结束前必须终止，无需手动管理joinHandle
# 解释代码
```rust
    let a = String::from("hello");
    thread::scope(|s| {
        let b = String::from("hello");
        for i in 0..5 {
            s.spawn(|| {
                thread::sleep(Duration::from_secs(1));
                println!("thread #{}", b);
            });
        }
    })
```
a可以打印，因为a的生命周期长，可以引用
b不行，因为b的生命周期比线程短，不可以引用
因此，要使用b就要转移所有权。
# into_iter和iter的区别
`into_iter` 和 `iter` 是 Rust 中迭代器的两种重要方法，它们的主要区别在于**所有权**的处理方式：

## 🔄 `iter()` - 借用迭代
```rust
let vec = vec![1, 2, 3];
for item in vec.iter() {  // 或者 &vec
    // item 是 &i32 类型（引用）
    println!("{}", item);
}
// vec 仍然可用，所有权没有被转移
println!("vec: {:?}", vec);  // ✅ 可以继续使用
```

**特点：**
- 产生元素的不可变引用 `&T`
- 不消耗原始集合
- 原始数据在迭代后仍然可用

## ➡️ `into_iter()` - 所有权转移迭代
```rust
let vec = vec![1, 2, 3];
for item in vec.into_iter() {
    // item 是 i32 类型（值）
    println!("{}", item);
}
// vec 不再可用，所有权已被转移
// println!("vec: {:?}", vec);  // ❌ 编译错误
```

**特点：**
- 消耗原始集合，获取元素的所有权
- 产生元素的值 `T`
- 原始数据在迭代后不再可用

## 📊 对比表格

| 特性 | `iter()` | `into_iter()` |
|------|----------|---------------|
| **所有权** | 借用 | 转移 |
| **返回类型** | `&T` | `T` |
| **消耗集合** | 否 | 是 |
| **后续使用** | 可以继续使用 | 不能再使用 |
| **常用场景** | 只读遍历 | 转换或消耗数据 |

## 🎯 使用场景示例

### 使用 `iter()`（只读访问）：
```rust
let numbers = vec![1, 2, 3, 4, 5];

// 计算总和而不消耗数据
let sum: i32 = numbers.iter().sum();
println!("Sum: {}, original: {:?}", sum, numbers);  // ✅ 仍然可用

// 查找元素
if numbers.iter().any(|&x| x == 3) {
    println!("Found 3!");
}
```

### 使用 `into_iter()`（消耗数据）：
```rust
let numbers = vec![1, 2, 3, 4, 5];

// 转换数据并获取所有权
let doubled: Vec<i32> = numbers.into_iter().map(|x| x * 2).collect();
// numbers 不再可用，但 doubled 是新集合
println!("Doubled: {:?}", doubled);

// 或者直接消耗
let strings = vec!["hello".to_string(), "world".to_string()];
for s in strings.into_iter() {
    println!("{}", s);  // 获取每个 String 的所有权
}
// strings 不再可用
```

## 🔄 隐式转换

在 `for` 循环中，Rust 会自动选择合适的方法：
```rust
let vec = vec![1, 2, 3];

// 等价于 vec.iter()
for item in &vec {  // 借用
    println!("{}", item);
}

// 等价于 vec.into_iter()  
for item in vec {   // 移动
    println!("{}", item);
}
```

## 💡 记忆技巧
- `iter()` → 我想**看**数据
- `into_iter()` → 我想**拿**数据

选择哪个取决于你是否还需要在迭代后使用原始集合。