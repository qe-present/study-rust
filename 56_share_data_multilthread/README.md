# 多线程共享数据
- Static
- Box::leak
- Arc<T>
1. Static
- 拥有 'static 生命周期的引用
- 只能用常量值初始化
- 代表一个内存地址，可以进行引用
- 程序结束不会drop
- 可以是mut
```rust
use std::thread;

static DATA: [i32; 5] =[1,2,3,4,5];
fn main() {
    let mut handles=Vec::new();
    for i in 1..20 {
        let h=thread::spawn(move || {
            println!("{DATA:#?}")
        });
        handles.push(h);
    }
    handles.into_iter().for_each(|h|{
        h.join().unwrap();
    })
    
}
```
可变的
```rust
use std::thread;

static mut COUNTER: u32 = 0;
fn main() {
    let mut handles=Vec::new();
    for i in 0..10000 {
        let h=thread::spawn(move || {
            unsafe { COUNTER += 1; }
        });
        handles.push(h);
    }
    handles.into_iter().for_each(|h|{
        h.join().unwrap();
    });
    println!("{:?}", unsafe{COUNTER});

}
```
2. Box::leak 
主动泄漏内存分配，释放Box所有权，并承诺永远不会drop它
```rust
use std::thread;


fn main() {
    let data:&'static [i32;5]=Box::leak(Box::new([1,2,3,4,5]));
    let mut handles=Vec::new();
    for i in 0..10000 {
        let h=thread::spawn(move || {
            println!("{data:?}");
        });
        handles.push(h);
    }
    handles.into_iter().for_each(|h|{
        h.join().unwrap();
    });

}
```
3. Arc<T>
- 原子引用计数
```rust
use std::sync::Arc;
use std::thread;


fn main() {
    let data=Arc::new(Box::new([1,2,3,4,5]));
    let mut handles=Vec::new();
    for i in 0..10000 {
        let local_data=data.clone();
        let h=thread::spawn(move || {
            println!("{local_data:?}");
        });
        handles.push(h);
    }
    handles.into_iter().for_each(|h|{
        h.join().unwrap();
    });

}
```