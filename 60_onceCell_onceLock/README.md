# OnceCell 与 OnceLock 示例

本项目演示了 Rust 标准库中 `OnceCell` 和 `OnceLock` 的使用方法。这些类型提供了一次性初始化的内存位置，用于在运行时计算并存储值。

## 项目结构

- `src/lib.rs` - 包含所有示例和单元测试

## OnceCell 示例

OnceCell 是单线程环境下的一次性初始化单元。它允许你将值的初始化推迟到首次访问时，并且只能被设置一次。

### 主要功能

1. **创建 OnceCell**
   ```rust
   let cell = OnceCell::new();
   ```

2. **设置值**
   ```rust
   let result = cell.set(String::from("hello world"));
   // result.is_ok() 表示设置成功
   // 第二次调用会失败，返回 Err
   ```

3. **获取值**
   ```rust
   let value = cell.get(); // 返回 Option<&T>
   ```

4. **惰性初始化**
   ```rust
   let result = cell.get_or_init(|| "hello world".to_string());
   // 如果尚未设置值，则使用闭包进行初始化
   - 设置成功返回 Ok
   - 设置失败返回 Err
   ```

5. **可变访问**
   ```rust
   if let Some(v) = cell.get_mut() {
       v.push_str("!");
   }
   // 注意：set() 返回 Err 后，仍可通过 get_mut() 获取可变引用
   ```

## 关键特性

- **一次性初始化**：只能设置一次，第二次调用 `set()` 会失败
- **惰性求值**：通过 `get_or_init()` 延迟初始化到首次访问时
- **内部可变性**：通过 `get_mut()` 可以修改已存储的值（如果可变）
- **线程不安全**：`OnceCell` 仅适用于单线程环境，多线程环境请使用 `OnceLock`

## 运行测试

```bash
cargo test
```

## 使用场景

- 缓存昂贵的计算结果
- 存储配置文件（只需加载一次）
- 全局状态的单次初始化
- 延迟初始化以提高启动性能

## OnceLock

`OnceLock` 是 `OnceCell` 的线程安全版本，提供相同但可以在多线程环境中安全使用的 API。

### 主要功能

1. **创建 OnceLock**
   ```rust
   static LOCK: OnceLock<usize> = OnceLock::new();
   ```

2. **惰性初始化（线程安全）**
   ```rust
   let value = LOCK.get_or_init(|| 12345);
   // 只会被调用一次，即使多个线程同时访问
   // 返回 &usize
   ```

3. **多线程安全的引用**
   ```rust
   assert_eq!(LOCK.get(), Some(&12345));
   // 返回 Option<&T>
   ```

### 关键特性

- **线程安全**：多个线程可以安全地并发访问
- **只初始化一次**：`get_or_init()` 保证闭包只被执行一次
- **静态变量**：非常适合用于全局静态变量的惰性初始化

### OnceList 示例：线程安全的数据结构

项目包含一个使用 `OnceLock` 实现的线程安全链表：

```rust
struct OnceList<T> {
    data: OnceLock<T>,           // 存储当前节点数据
    next: OnceLock<Box<OnceList<T>>>  // 存储下一个节点
}
```

**核心方法：**

1. **创建新链表**
   ```rust
   const fn new() -> OnceList<T> {
       OnceList {
           data: OnceLock::new(),
           next: OnceLock::new(),
       }
   }
   ```

2. **插入元素（线程安全）**
   ```rust
   fn push(&self, val: T) {
       // 如果当前节点已被占用，转移到下一个节点
       if let Err(val) = self.data.set(val) {
           let next_node = self.next.get_or_init(|| {
               Box::new(OnceList::new())
           });
           next_node.push(val);
       }
   }
   ```
   - `set()` 返回 `Err(val)` 表示当前节点已有数据，需要递归到下一个节点
   - `get_or_init()` 确保下一个节点只在需要时才创建

3. **查找元素（线程安全）**
   ```rust
   fn contains(&self, val: &T) -> bool
   where T: PartialEq {
       self.data.get()
           .map(|item| item == val)
           .filter(|v| *v)
           .unwrap_or_else(|| {
               self.next.get()
                   .map(|next_node| next_node.contains(val))
                   .unwrap_or(false)
           })
   }
   ```
   - 在当前节点查找，找不到时递归到下一个节点
   - 使用函数式编程风格（map + filter + unwrap_or_else）

#### 代码详解

让我们逐行分析这个函数式链条：

```rust
self.data.get()           // 1. 获取当前节点的数据，返回 Option<&T>
    .map(|item| item == val)  // 2. 如果有数据，比较是否等于目标值
                               //    返回 Option<bool>（Some(true/false) 或 None）
    .filter(|v| *v)          // 3. 过滤：只保留 true 的值
                               //    如果是 Some(false)，也变成 None
    .unwrap_or_else(|| {      // 4. 如果前面的结果是 None（没找到或找到但不相等）
        self.next.get()       //    5. 获取下一个节点，返回 Option<&Box<OnceList<T>>>
            .map(|next_node| next_node.contains(val))  // 6. 递归调用 contains
            .unwrap_or(false)  // 7. 如果没有下一个节点，返回 false
    })
```

##### 执行流程示例

```rust
// 查找 [10] -> [20] -> [30] 中是否包含 20

LIST.contains(&20);

↓
self.data.get()        // Some(&10)
  .map(...)            // Some(10 == 20) → Some(false)
  .filter(...)         // Some(false) → None（被过滤掉）
  .unwrap_or_else(...)

↓ // 进入闭包
self.next.get()        // Some(&Box<[20] -> [30]>)
  .map(...)            // recursively: [20] -> [30].contains(&20)

    ↓ // 递归调用
    self.data.get()    // Some(&20)
      .map(...)        // Some(20 == 20) → Some(true)
      .filter(...)     // Some(true) → Some(true)
      .unwrap_or(...)  // 得到 true，返回 true

  .unwrap_or(false)   // true
.unwrap_or_else(...)  // true

最终结果: true
```

##### 几种情况的返回值

| 当前节点 | 查找值 | map后的值 | filter后的值 | 是否递归 | 最终结果 |
|---------|-------|----------|-------------|---------|---------|
| Some(10) | 10 | Some(true) | Some(true) | 否 | **true** |
| Some(10) | 20 | Some(false) | None | 是 | 看下一个节点 |
| None | 10 | None | None | 是 | 看下一个节点 |
| None（无下一个） | 任何 | None | None | 否 | **false** |

### 多线程并发测试

示例演示了在多线程环境下安全地使用 `OnceList`：

```rust
static LIST: OnceList<u32> = OnceList::new();
static COUNTER: AtomicU32 = AtomicU32::new(0);
const LEN: u32 = 1000;

// 多个线程并发插入数据
let _ = scope(|scope| {
    for _ in 0..thread::available_parallelism().unwrap().get() {
        scope.spawn(|_| {
            while let i @ 0..LEN = COUNTER.fetch_add(1, Ordering::Relaxed) {
                LIST.push(i)
            }
        });
    }
});

// 验证所有 0-999 的数字都被成功插入
for i in 0..LEN {
    assert!(LIST.contains(&i));
}
```

**关键点：**
- 使用 `AtomicU32` 作为线程安全的计数器
- `while let i @ 0..LEN = ...` 只有在值在范围内时才继续循环
- 多个线程并发向同一个 `OnceList` 插入数据，线程安全
- 每个线程自动处理不同的数字，无数据竞争

### OnceLock vs OnceCell 对比

| 特性 | OnceCell | OnceLock |
|------|---------|----------|
| **线程安全** | ❌ 否 | ✅ 是 |
| **静态变量** | ❌ 否（需要 OnceLock） | ✅ 是 |
| **使用场景** | 单线程 | 多线程 |
| **API** | 相同 | 相同 |
| **性能** | 略高（无锁） | 使用内部同步机制 |

## 版本要求

- Rust 1.70+ (for OnceCell/OnceLock)
- Edition 2024

# while let i @ 0..LEN  是什么意思
这是一个 Rust 的**模式匹配**语法，结合了 `while let` 和 **绑定模式（binding mode）**。

## 语法分解

```rust
while let i @ 0..LEN = expr {
    // 使用 i
}
```

### 1. `@` 绑定（Binding）
- `i @ pattern` 表示"匹配这个模式，并将匹配的值**完整绑定**到变量 `i`"
- `i` 会获得 `expr` 的**实际值**，而不仅仅是范围检查的结果

### 2. `0..LEN` 范围模式
- 这是**半开区间** `[0, LEN)`，包含 0 但不包含 LEN
- 只有当 `expr` 的值在这个范围内时，模式匹配才成功

### 3. `while let` 循环
- 只要模式匹配成功，循环就会继续
- 一旦匹配失败（值超出范围），循环立即终止

## 执行流程

```rust
let value = COUNTER.fetch_add(1, Ordering::Relaxed);

// 等价于：
match value {
    i @ 0..LEN => {
        // 匹配成功，执行循环体
        LIST.push(i);
        continue; // 继续下一次循环
    }
    _ => {
        // 匹配失败（i >= LEN），退出循环
        break;
    }
}
```

## 关键点

| 特性 | 说明 |
|------|------|
| **原子性** | `fetch_add` 返回的是**递增前的旧值** |
| **范围检查** | 只有 `< LEN` 的值才会被处理 |
| **值绑定** | `i` 可以在循环体内直接使用，无需再次解包 |
| **自动退出** | 当计数器达到 `LEN` 时，所有线程自动停止 |

## 对比：普通 while 循环

**不推荐**的写法（需手动解包）：
```rust
loop {
    let i = COUNTER.fetch_add(1, Ordering::Relaxed);
    if i < LEN {
        LIST.push(i);
    } else {
        break;
    }
}
```

**推荐**的 `while let` 写法（更简洁、Rust 风格）：
```rust
while let i @ 0..LEN = COUNTER.fetch_add(1, Ordering::Relaxed) {
    LIST.push(i);
}
```

这种写法在 Rust 中非常常见，特别是在需要**边提取值边进行范围/类型检查**的场景。