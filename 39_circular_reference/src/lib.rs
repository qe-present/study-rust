use std::rc::{Rc,Weak};
use std::cell::RefCell;
#[derive(Debug)]
struct Node{
    value:i32,
    children:RefCell<Vec<Rc<Node>>>,
    parent:RefCell<Weak<Node>>,
}

impl Node{
    fn new(value:i32)->Rc<Node>{
        Rc::new(Node{
            value,
            children:RefCell::new(vec![]),
            parent:RefCell::new(Weak::new()),   
        })
    }
}
/// cargo test -- --nocapture
#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_node(){
        let leaf=Rc::new(Node{
            value:3,
            children:RefCell::new(vec![]),
            parent:RefCell::new(Weak::new()),
        });
        println!("leaf parent={:?}",leaf.parent.borrow().upgrade());
        let branch=Rc::new(Node{
            value:5,
            children:RefCell::new(vec![Rc::clone(&leaf)]),
            parent:RefCell::new(Weak::new()),
        });
        *leaf.parent.borrow_mut()=Rc::downgrade(&branch);
        println!("leaf parent={:?}",leaf.parent.borrow().upgrade());    
        println!("branch strong={},weak={}",Rc::strong_count(&branch),Rc::weak_count(&branch));
        println!("leaf strong={},weak={}",Rc::strong_count(&leaf),Rc::weak_count(&leaf));
        
    }
}   
