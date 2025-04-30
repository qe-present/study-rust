// 实现了Deref trait的类型可以被当作引用来使用
use std::ops::Deref;

struct MyBox<T>(T);
impl<T> MyBox<T>{
    fn new(x:T)->Self{
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///
/// # Example
/// ```
/// let x=5;
///     let y=Box::new(x);// Box<T>实现了Deref trait let y=&x;
///     assert_eq!(5,x);
///     assert_eq!(5,*y);
///```
/// 


/// 使用hello
/// # Example
/// ```Rust
///   let a=MyBox::new(String::from("hello"));
///   hello(&a)
/// ```
/// 隐式引用 &MyBox <String> -> &String -> &str(String 实现了deref)
/// 隐式解引用转换，实现trait Deref 的类型的引用可用转换为另一个类型的引用
fn hello(name:&str){
    println!("hello {}",name)
}
/// 三种解引用的转换
/// &T->&U(Deref)
/// &mut T->&mut U(DerefMut
/// &mut t->&U (Deref)
fn main(){
    let a=String::from("hello");
    hello(&a)
}