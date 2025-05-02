
use std::rc::Rc;
pub enum List{
    Cons(i32, Rc<List>),
    Nil,
}
use List::{Cons, Nil};
/// Rc<T> 是一个智能指针，它允许你在多个地方共享同一块内存
/// Rc 通过不可变引用来实现共享内存
/// std::rc::Rc
/// strong_count() 返回引用计数
/// Rc::clone() 返回一个新的Rc指针

fn main() {
    let a=Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count {}",Rc::strong_count(&a));
    let b=Rc::clone(&a);
    println!("count {}",Rc::strong_count(&b));
    let c=Rc::clone(&a);
    println!("count {}",Rc::strong_count(&c));
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn question_1(){
        let mut n=1;
        let b=Box::new(&mut n);
        **b+=1;
        println!("{}",n)
    }
    #[test]
    fn question_2(){
        let n=Rc::new(1);
        let mut n2=Rc::clone(&n);
        // *n2+=1;因为 Rc<T> 是不可变的智能指针，无法直接修改其内部数据。
        println!("{}",n);
    }
}