# Rust 错误处理

Rust 的错误处理机制非常强大，它通过 `Result` 和 `Option` 枚举类型来处理错误，而不是使用异常机制。

## 不可恢复错误与 panic!

### panic! 宏

```rust
fn main() {
    panic!("crash and burn");
}
```

### 使用 panic! 的 backtrace

```rust
fn main() {
    let v = vec![1, 2, 3];
    v[99]; // 这会导致 panic!
}
```

## 可恢复错误与 Result

### Result 枚举

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### 使用 match 表达式处理 Result

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("打开文件失败: {:?}", error)
        },
    };
}
```

### 匹配不同的错误

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("创建文件失败: {:?}", e),
            },
            other_error => panic!("打开文件失败: {:?}", other_error),
        },
    };
}
```

### 失败时 panic 的简写：unwrap 和 expect

```rust
// 使用 unwrap
let f = File::open("hello.txt").unwrap();

// 使用 expect
let f = File::open("hello.txt").expect("无法打开文件");
```

## 传播错误

### 使用 ? 运算符传播错误

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

### ? 运算符与 from 函数

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```

## 何时使用 panic!

### 示例、代码原型和测试

```rust
fn main() {
    let home: IpAddr = "127.0.0.1".parse().unwrap();
}
```

### 错误处理指导原则

1. 在示例代码中
2. 在原型代码中
3. 在测试代码中
4. 当你比编译器知道更多信息时

### 创建自定义类型进行验证

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("猜测值必须在1到100之间，得到的是{}", value);
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
```

## 将错误组合在一起

### 使用自定义错误类型

```rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ParsePosNonzeroError {
    kind: ParsePosNonzeroErrorKind,
}

#[derive(Debug)]
pub enum ParsePosNonzeroErrorKind {
    CreationError,
    ParseIntError,
}

impl fmt::Display for ParsePosNonzeroError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("解析正数失败")
    }
}

impl Error for ParsePosNonzeroError {}
```

### 使用 Box<dyn Error>

```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
}
```

## 错误处理最佳实践

1. **使用 Result 而不是 panic!**
   - 对于可恢复的错误使用 Result
   - 对于不可恢复的错误使用 panic!

2. **使用 ? 运算符传播错误**
   - 简化错误传播代码
   - 保持代码的可读性

3. **创建自定义错误类型**
   - 提供更好的错误信息
   - 实现 Error trait

4. **使用类型系统防止错误**
   - 创建新类型进行验证
   - 使用类型约束确保正确性

5. **在测试中使用 panic!**
   - 测试失败时提供清晰的错误信息
   - 使用 expect 提供有意义的错误消息 
## 学习总结

1. Rust将错误分为两大类：可恢复错误和不可恢复错误
2. 可恢复错误使用`Result<T, E>`枚举处理
3. 不可恢复错误使用`panic!`宏处理，程序会终止执行
4. 当`panic!`发生时，程序默认会展开栈，清理数据
5. 可以通过`Cargo.toml`配置让程序直接终止，减小二进制文件大小
6. `Result<T, E>`定义：`enum Result<T, E> { Ok(T), Err(E) }`
7. 处理`Result`的方法：`match`、`unwrap`、`expect`、`?`运算符等
8. `unwrap`在`Err`时会自动调用`panic!`
9. `expect`允许自定义`panic!`信息
10. `?`运算符简化错误传播：`let f = File::open("hello.txt")?;`
11. `?`运算符只能用于返回`Result`或`Option`类型的函数
12. 可以自定义错误类型并实现`std::error::Error`特性
13. 通常使用`thiserror`或`anyhow`等crate简化错误处理 

---

| [上一页：哈希Map](../23_hashMap/23_hashMap.md) | [下一页：泛型](../25_generic/25_generic.md) |
|------------------------|------------------------|