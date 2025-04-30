/*
BOX智能指针，
引用进借用数据，但不是拥有数据
比如 Box、Rc、Ref、RefMut、RefCell
Box特点
允许将数据存储在Heap上
 */
enum List{
    Cons(i32, Box<List>),
    Nil,
}
fn main() {
    let l=List::Cons(5, Box::new(List::Cons(10, Box::new(List::Nil))));
}
#[cfg(test)]
mod test {
    #[test]
    fn question_1(){
        let mut n=1;
        let b=Box::new(&mut n);
        **b+=1;
        println!("{}",n)
    }
    
}
