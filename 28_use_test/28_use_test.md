# 测试

单元测试和集成测试相关学习笔记。

## 学习总结

1. Rust的测试是带有`test`属性标注的函数
2. 测试函数通常使用`assert!`、`assert_eq!`和`assert_ne!`宏来验证结果
3. 单元测试使用`#[cfg(test)]`属性标记测试模块，通常放在与被测代码相同的文件中
4. 集成测试位于项目根目录的`tests`目录下，每个文件被视为单独的crate
5. 使用`cargo test`命令运行所有测试
6. 使用`cargo test test_name`运行特定测试
7. 使用`cargo test module_name`运行特定模块中的测试
8. 测试默认并行运行，可以使用`--test-threads=1`选项强制串行运行
9. 默认情况下，测试中的输出会被捕获，使用`--nocapture`选项可以显示输出
10. 可以使用`#[should_panic]`属性测试代码是否按预期panic
11. 使用`Result<(), E>`作为测试函数返回类型可以使用`?`运算符简化测试

## 单元测试示例

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works_result() -> Result<(), String> {
        if add(2, 2) == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```

## 集成测试示例

```
// 在tests/integration_test.rs中
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```

---

| [上一页：生命周期](../27_lifetime/27_lifetime.md) | [下一页：Cargo测试](../29_use_cargo_test/29_use_cargo_test.md) |
|------------------------|------------------------| 