/// RefCell 是内部可变性的一种实现方式
/// 允许在不可变引用的情况下修改数据
/// RefCell<T> 是一个智能指针，它允许你在运行时检查借用规则
/// RefCell 只能在单线程中使用
/// RefCell 使用的时机 遵守借用规则，但编译器无法理解或保证时
/// std::cell::RefCell
use std::cell::RefCell;
fn use_refcell(){
    let data=RefCell::new(5);
    *data.borrow_mut()+=1;
    println!("{}",data.borrow());
}
// test double
// mock object
/// Example
/// ```rust
/// use std::cell::RefCell;
/// fn use_refcell(){
///     let data=RefCell::new(5);
///     let data2=data.borrow_mut();
///     *data2+=1;
///     println!("{}",data.borrow());
/// }
/// ```
/// 代码有错，
/// RefCell 是 Rust 中用于在运行时管理借用规则的类型。
/// 它允许多个不可变借用（borrow）或一个可变借用（borrow_mut），
/// 但不能同时存在可变借用和不可变借用。
/// # Alter
/// ```rust
/// use std::cell::RefCell;
/// fn use_refcell(){
///    let data=RefCell::new(5);
///   {
///     let mut data2=data.borrow_mut();
///         *data2+=1;
///   }
///  println!("{}",data.borrow());
/// }
/// 
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn question_1(){
        let data=RefCell::new(5);
        {
            let mut data2=data.borrow_mut();
            *data2+=1;
        }
        println!("{}",data.borrow());
    }
    #[test]
    fn question_2(){
        use_refcell();
    }
}
