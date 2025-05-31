/// Drop trait 是在prelude中定义的
/// Drop trait 是一个特殊的trait，它允许你在对象被销毁时执行一些清理操作
/// 先调用的，后释放
/// ```rust
/// 
///  #[derive(Debug)]
///  struct Point{
///      data:i32
/// }
/// impl Drop for Point{
///     fn drop(&mut self) {
///        println!("Point {} is dropped",self.data);
///     }
/// }
/// fn main.rs() {
///     let p=Point{data:1};
///      let p1=Point{data:2};
///     println!("{:?}",p);
///    println!("{:?}",p1);
///  }
///
/// ```
/// 运行结果：
/// ```text
/// Point { data: 1 }
/// Point { data: 2 }
/// Point 2 is dropped
/// Point 1 is dropped
/// ```
struct Point{
    data:i32
}

/// std::dem::drop
/// drop函数是一个特殊的函数，它允许你在对象被销毁时执行一些清理操作
/// drop是在prelude中定义的
/// # Example
/// ```rust
/// fn main.rs() {
///     let p=Point{data:1};
///      let p1=Point{data:2};
///      drop(p);
/// }
/// ```
/// 结果如下
///
/// ![asset](../asset/img.png)
/// 
/// 
struct Rectangle{
    width:i32,
    height:i32
}