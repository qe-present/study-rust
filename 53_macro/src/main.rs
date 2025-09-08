/// 宏  
/// 1 54_声明宏
/// 2 过程宏  
///   - 自定义 derive 宏
///   - 属性宏
///   - 函数宏
/// 3 宏和函数的区别
///    - 宏在编译时展开，而函数在运行时调用
///    - 宏可以接受任意类型的参数，而函数只能接受特定类型的参数
/// 
/// 4 vec! 宏
/// ```rust
/// #【macro_export】
/// macro_rules! vec {
///    ($($x:expr),*) => {
///        {
///            let mut temp_vec = Vec::new();
///           $(
///               temp_vec.push($x);
///           )*
///             temp_vec
///         }
///     };
/// macro_export 表示该宏可以在其他crate中使用
/// macro_rules! 是声明宏的关键字
/// {} 表示宏的主体
/// $(...) 是Rust宏 中表示重复模式的语法
/// $x:expr 匹配任意Rust表达式，并将其绑定到变量 $x
/// ,* 表示匹配零个或多个逗号分隔的表达式
/// 5 过程宏
/// 定义过程宏接受一个TokenStream作为输入，并返回一个TokenStream作为输出
/// 5.1 派生宏 proc_macro_derive
/// 5.2 属性宏 proc_macro_attribute
/// 5.3 函数宏 proc_macro
/// 
use hello_macro_derive::HelloMacro;
use hello_macro::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;
fn main() {
    Pancakes::hello_macro(); // 调用宏生成的代码
}