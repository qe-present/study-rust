/// 1 使用newtype模式
///  - Rust允许使用新类型模式来创建新的类型
///  ```rust
///  struct Millimeters(u32); 
///  struct Meters(u32);
///  ```
/// 2. 使用类型别名
///  - 使用type关键字可以定义类型别名
///  ```rust
/// type Millimeters = u32;
/// type Meters = u32;
/// ```
/// 二者是同一个类型
/// type Result<T> = Result<T, E>;
/// 3. 永不返回的类型 Never type
///  ！ 
/// fn never_fn() -> ! {
/// }
/// continue;panic!();loop {}
/// 4 动态大小类型和Sized trait
/// 大小在编译时未知的类型称为动态大小类型（DST）
/// 动态大小被放到box、引用或切片中
/// 5 高级函数
/// 6 Function Pointers
/// fn 是一个函数指针类型
/// 函数指针实现l所有三种函数trait Fn、FnMut和FnOnce
/// 7 返回闭包
/// 使用impl Trait 
/// 使用trait object
fn main() {
    
}