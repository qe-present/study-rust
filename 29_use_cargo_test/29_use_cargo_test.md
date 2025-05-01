# Cargo测试

Cargo测试相关学习笔记。

## 学习总结

1. Cargo不仅是Rust的构建工具，也是测试执行和管理工具
2. `cargo test`命令自动寻找和运行项目中的所有测试
3. Cargo测试选项分为两部分：`cargo test`的选项和传递给测试二进制文件的选项
4. 使用`--`分隔两部分选项：`cargo test -- --nocapture`
5. 常用Cargo测试选项：
   - `--test-threads=N`：设置测试线程数（默认并行）
   - `--show-output`：显示测试期间的标准输出
   - `--ignored`：只运行被忽略的测试
   - `--include-ignored`：运行所有测试，包括被忽略的
   - `--test <name>`：只运行指定的集成测试
6. 使用`#[ignore]`属性可以标记暂时不运行的测试
7. 可以按名称过滤测试：`cargo test add`运行名称中包含"add"的所有测试
8. 集成测试放在`tests`目录，该目录与`src`目录平级
9. 每个集成测试文件都是独立的crate，需要导入被测库
10. 可以在`tests`目录下创建子目录组织测试辅助函数，Cargo不会将子目录视为测试文件

## 测试选项示例

```bash
# 运行所有测试
cargo test

# 只运行名称中包含"add"的测试
cargo test add

# 只运行被忽略的测试
cargo test -- --ignored

# 单线程运行测试并显示输出
cargo test -- --test-threads=1 --nocapture

# 只运行特定的集成测试文件
cargo test --test integration_test
```

## 测试组织示例

```
my_project/
├── Cargo.toml
├── Cargo.lock
├── src/
│   └── lib.rs         # 库代码
│   └── main.rs        # 二进制代码
└── tests/
    ├── common/        # 测试辅助模块
    │   └── mod.rs     # 共享测试功能
    ├── integration_test.rs  # 集成测试1
    └── api_test.rs          # 集成测试2
```

## 测试辅助模块示例

```rust
// tests/common/mod.rs
pub fn setup() -> TestEnvironment {
    // 设置测试环境...
    TestEnvironment { /* ... */ }
}

// tests/integration_test.rs
mod common; // 导入辅助模块

#[test]
fn test_with_common_setup() {
    let env = common::setup();
    // 使用设置好的测试环境进行测试...
}
```

---

| [上一页：测试](../28_use_test/28_use_test.md) | [下一页：命令行程序](../30_minigrep/30_minigrep.md) |
|------------------------|------------------------| 