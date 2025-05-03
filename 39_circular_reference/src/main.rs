mod weak_example;

use std::cell::RefCell;
use std::rc::Rc;
use crate::List::{Cons,Nil};
#[derive(Debug)]
enum List{
    Cons(i32,RefCell<Rc<List>>),
    Nil,
}
impl List{
    fn tail(&self)->Option<&RefCell<Rc<List>>>{
        match self{
            Cons(_,item)=>Some(item),
            Nil=>None,
        }
    }
}
/// 循环引用
/// 在Rust中，循环引用是可能的，但需要小心处理。
/// 当使用RefCell和Rc时，循环引用可能导致内存泄漏，因为Rc无法自动释放资源。
/// rust 无法自动检测循环引用
/// 应使用自动化测试、代码审查等方法来检测循环引用
/// 1、使用Weak<T>来打破循环引用    

fn main(){
    println!("循环引用示例 - 展示引用循环问题:");
    
    let a=Rc::new(Cons(5,RefCell::new(Rc::new(Nil))));
    println!("a initial rc count={}",Rc::strong_count(&a));
    println!("a next item ={:?}",a.tail());

    let b=Rc::new(Cons(10,RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation={}",Rc::strong_count(&a));
    println!("b initial rc count={}",Rc::strong_count(&b));
    println!("b next item ={:?}",b.tail());
    if let Some(link)=a.tail(){
        *link.borrow_mut()=Rc::clone(&b);
    }
    println!("a rc count after changing a={}",Rc::strong_count(&a));
    println!("b rc count after changing a={}",Rc::strong_count(&b));
    println!("注意: 现在已经形成了循环引用 a->b->a");
    println!("在程序结束时，这些引用计数永远不会降为0，导致内存泄漏");
    
    // 运行使用Weak<T>解决循环引用的示例
    weak_example::run_weak_example();
}
