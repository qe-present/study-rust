# 命令行程序

命令行程序开发相关学习笔记。

## 学习总结

1. 使用`std::env::args()`获取命令行参数，返回迭代器
2. 命令行参数的第一个值是程序名称本身
3. 使用`std::fs`模块读取文件
4. 使用`expect`处理可能的错误，但在实际应用中应该使用更好的错误处理方法
5. 将代码拆分到不同的模块中，提高可维护性
6. 使用`std::error::Error` trait处理不同类型的错误
7. 使用`Box<dyn Error>`表示任何类型的错误
8. 将功能代码移入`lib.rs`，保持`main.rs`简洁
9. 使用环境变量（通过`std::env::var()`）配置程序行为
10. 使用测试驱动开发（TDD）方法实现核心功能
11. 标准输出（`println!`）和标准错误（`eprintln!`）的区别
12. 使用`Result<T, E>`类型处理可能失败的操作

## 程序结构示例

```
minigrep/
├── Cargo.toml
├── src/
│   ├── main.rs      // 程序入口点
│   └── lib.rs       // 核心功能实现
└── poem.txt         // 测试文件
```

## 代码示例

```rust
// src/main.rs
use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("参数解析错误: {}", err);
        process::exit(1);
    });
    
    if let Err(e) = minigrep::run(config) {
        eprintln!("应用错误: {}", e);
        process::exit(1);
    }
}

// src/lib.rs
use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("参数不足");
        }
        
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        
        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    
    for line in results {
        println!("{}", line);
    }
    
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
```

---

| [上一页：Cargo测试](../29_use_cargo_test/29_use_cargo_test.md) | [下一页：循环与迭代器](../31_loop_vs_iterator/31_loop_vs_iterator.md) |
|------------------------|------------------------| 