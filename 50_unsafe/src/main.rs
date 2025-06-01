use std::slice::from_raw_parts_mut;
/// 5 件事
/// 1 解引用原始指针
fn one(){
    let mut x: i32 = 42;
    let x1=&raw const x;
    let x2=&raw mut x;
    unsafe {
        println!("x1: {}", *x1); // 解引用原始指针
        println!("x2: {}", *x2);
    }
}
fn main() {
    // 1 解引用原始指针
    one();
    // 2 调用不安全的函数/方法
    unsafe {
        two();
    }
    let mut v=vec![1,2,3,4,5,6];
    let r=&mut v;
    let (a, b) = split_as_mut(r, 3);
    assert_eq!(a,&mut [1,2,3]);
    assert_eq!(b,&mut [4,5,6]);
    // 3 使用extern调用外部代码
    let res=use_abs(-10);
    assert_eq!(res,10);
    // 4 访问或修改可变静态变量
    use_counter();
    // 5 使用不安全的 trait
    use_unsafe_trait();
    // 6 访问union的字段
    use_union();
}
/// 2 调用不安全的函数/方法
/// unsafe 函数 添加了unsafe
/// 包含unsafe代码的函数不一定要标记unsafe
/// 用安全函数封装unsafe代码是常见的抽象模式
unsafe fn two(){
    println!("hello world");
}
fn split_as_mut(value:&mut [i32],mid:usize)-> (&mut [i32],&mut [i32]) {
    assert!(mid <= value.len());
    let ptr = value.as_mut_ptr();
    unsafe {
        (
            from_raw_parts_mut(ptr, mid),
            from_raw_parts_mut(ptr.add(mid), value.len() - mid),
        )
    }
    
}

unsafe extern "C"{ // ABI
    safe fn abs(x: i32) -> i32;
}
/// 3 使用extern调用外部代码
/// - 外部函数接口FFI__Foreign Function Interface
/// - Rust 代码需要调用其他语言编写的代码时
/// - 使用extern可以创建和使用外部函数接口
/// - FFI 使一种语言能够调用另一种语言的函数或方法
fn use_abs(x: i32) -> i32 {
    unsafe { 
        println!("abs({})={}", x, abs(x));
        abs(x)
    }
}
/// 4 访问或修改可变静态变量
/// 
/// - 静态变量在程序的整个生命周期内存在
/// - 静态变量可以是可变的
/// - 访问或修改可变静态变量需要使用unsafe
static mut COUNTER: i32 = 0;
unsafe fn increment_counter() {
    unsafe {
        COUNTER += 1;
    }
}
fn use_counter() {
    unsafe {
        increment_counter();
        println!("counter: {}", *(&raw const COUNTER));
    }
}
/// 5 使用不安全的 trait
/// - Rust 允许定义不安全的 trait
/// - 实现不安全的 trait 需要使用 unsafe
/// - 不安全的 trait 通常用于需要绕过 Rust 的安全检查的场景
unsafe trait UnsafeTrait {
    fn unsafe_method(&self);
}
unsafe impl UnsafeTrait for i32 {
    fn unsafe_method(&self) {
        unsafe {
            println!("Unsafe method called on: {}", *(&raw const self));
        }
    }
}
fn use_unsafe_trait() {
    let x: i32 = 42;
    unsafe {
        x.unsafe_method();
    }
}
/// 6 访问union的字段
/// - Rust 允许定义 union
/// - union 是一种特殊的数据结构
/// - union 的字段可以是不同类型z
union Converter {
    int_value: u32,
    bytes: [u8; 4],
}
fn use_union() {
    let converter = Converter { int_value: 0x12345678 };
    unsafe {
        println!("Bytes: {:X} {:X} {:X} {:X}", 
            converter.bytes[0], converter.bytes[1], converter.bytes[2], converter.bytes[3]);
    }
}