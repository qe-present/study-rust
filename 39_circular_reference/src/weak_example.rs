use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::fmt;

// 使用Weak<T>解决循环引用问题

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn new(value: i32) -> Rc<Node> {
        Rc::new(Node {
            value,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        })
    }
    
    fn add_child(&self, child: &Rc<Node>) {
        // 将子节点添加到children
        self.children.borrow_mut().push(Rc::clone(child));
        // 将父节点设置为当前节点的弱引用
        *child.parent.borrow_mut() = Rc::downgrade(child);
    }
}

pub fn run_weak_example() {
    println!("\n运行Weak<T>示例来解决循环引用问题:");
    
    // 创建一个父节点
    let parent = Node::new(1);
    println!("父节点创建后的强引用计数: {}", Rc::strong_count(&parent));
    
    {
        // 创建一个子节点
        let child = Node::new(2);
        println!("子节点创建后的强引用计数: {}", Rc::strong_count(&child));
        
        // 建立父子关系
        parent.add_child(&child);
        
        println!("添加子节点后父节点的强引用计数: {}", Rc::strong_count(&parent));
        println!("添加到父节点后子节点的强引用计数: {}", Rc::strong_count(&child));
        
        // 通过子节点获取父节点
        let parent_ref = child.parent.borrow().upgrade();
        match parent_ref {
            Some(p) => println!("通过子节点获取的父节点值: {}", p.value),
            None => println!("父节点已被释放"),
        }
        
        // 子节点会在块结束时被释放
        println!("子节点即将离开作用域");
    }
    
    // 子节点已被释放，但父节点仍然存在
    println!("子节点离开作用域后的父节点强引用计数: {}", Rc::strong_count(&parent));
    
    // 显示父节点的子节点数量
    println!("父节点的子节点数量: {}", parent.children.borrow().len());
    
    // 尝试通过父节点访问第一个子节点
    if let Some(child) = parent.children.borrow().get(0) {
        println!("通过父节点访问的子节点值: {}", child.value);
    }
} 