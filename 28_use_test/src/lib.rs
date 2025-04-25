/*
assert 宏
assert_eq 相等
assert_ne 不相等
测试panic should_panic  should_panic(expected = "xxx") 预期 panic 信息
测试中使用Result 
断言是否返回Err，不应该使用Result，使用asset!
 */

fn add_two(a:usize)->usize {
    a + 2
}
#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    fn  test_add_two()->Result<(),String> {
        let res=add_two(2);
        // 断言是否相等
        if res ==4{
            Ok(())
        }else { 
            // 断言失败，返回错误信息
            Err(format!("预期值为4，实际值为{}",res))
        }
        
    }
    
}