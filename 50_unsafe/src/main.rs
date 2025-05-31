/// 5 件事
/// 1 解引用原始指针
fn main() {
    let mut x: i32 = 42;
    let x1=&raw const x;
    let x2=&raw mut x;
    unsafe {
        println!("x1: {}", *x1); // 解引用原始指针
        println!("x2: {}", *x2);
    }
    
}