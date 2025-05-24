/// Future trait 
/// pub trait Future {
///     type Output;
///     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
/// enum Poll<T> {
///    Ready(T),
///   Pending,
/// }
/// Pin & Unpin
/// Pin 是针对（类）指针类型（如 Box、Rc、&mut 等）的一种封装，
/// Pin 本身不是一个指针类型，用来约束指针使用的工具
fn main() {
    
}