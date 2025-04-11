// 打印语句出现 cargo test -- --show-output
// cargo test test_prints_and_return_10 只运行指定的测试,模糊搜索
// #[ignore] 忽略测试
// cargo test -- --ignored 运行所有被忽略的测试
// cargo test -- --include-ignored 运行所有测试，包括被忽略的测试
fn prints_and_return_10(a:i32)->i32{
    println!("a is {}", a);
    return 10;
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_prints_and_return_10() {
        let result = prints_and_return_10(5);
        assert_eq!(result, 10);
    }
    #[test]
    #[ignore]
    fn test_prints_and_return_5_with_negative() {
        
        let result = prints_and_return_10(5);
        assert_eq!(result, 10);
    }
}