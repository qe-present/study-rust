/*
运行命令cargo doc 会生成文档
/// 是文档注释
 运行cargo doc --open会打开文档
 Example Panic Errors Safety 常见的Sections
 可以使用crate test
 //!为模块文档注释 出现在文档的开头

 */

//! # publish_crate
//! 测试publich crate
/// Add one to the given number
/// # Example
///
/// ``` rust
/// use publish_crate::add_one;
/// let x = 5;
/// let y = add_one(x);
/// assert_eq!(y, 6);
///```
/// This function takes an integer and returns the integer plus one.
///
///
pub fn add_one(x: i32) -> i32 {
    x + 1
}
