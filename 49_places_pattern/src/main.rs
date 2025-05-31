/// 模式匹配
/// 模式匹配是 Rust 中的一个强大特性，它允许我们根据数据的结构来执行不同的操作。
/// 模式匹配可以用于解构数据结构、提取值以及执行条件逻辑。
/// 在 Rust 中，模式匹配通常使用 `match` 关键字来实现。
///  if let 语句也可以用于模式匹配，但它更适合于简单的情况。
///  while let 语句可以用于循环中进行模式匹配。
/// let pattern = value; 语句可以用于将值与模式进行匹配。
/// 模式有两种形态 1. 不可反驳模式 2. 可反驳模式
/// 不可反驳模式：使用于所有可能的值，let x=5 中的 x 就是一个不可反驳模式。
/// 可反驳模式：使用于某些可能的值，if let Some(x) = option 中的 Some(x) 就是一个可反驳模式。
fn main() {
    // 匹配字面量
    let x = 5;
    match x {
        1 => println!("x is one"),
        2 => println!("x is two"),
        3 => println!("x is three"),
        4 => println!("x is four"),
        5 => println!("x is five"),
        _ => println!("x is something else"), // _ 是一个通配符，匹配所有其他情况
    }
    // 匹配命名变量
    let x=Some(5);
    let y=10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y= {}", y),
        _ => println!("Other value"),
    }
    println!("x: {:?},y:{}",x,y);
    // 多模式匹配 1|2
    // 匹配范围 1..=5
    // 匹配解构结构体  
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p; // 解构结构体
    println!("Point coordinates: x = {}, y = {}", x, y);
    match p {
        Point { x: 0, y: 0 } => println!("Origin"),
        Point { x, y: 0 } => println!("On the x-axis at {}", x),
        Point { x: 0, y } => println!("On the y-axis at {}", y),
        Point { x, y } => println!("Point at ({}, {})", x, y),
    }
    // 忽略值 _
    // 忽略剩余的部分 。。
    //匹配守卫
    let num=Some(4);
    match num {
        Some(x) if x < 5 => println!("x is less than 5: {}", x),
        Some(x) => println!("x is 5 or more: {}", x),
        None => println!("No value"),
    }
    // @绑定
    let x = 5;
    match x {
        1 => println!("x is one"),
        n @ 2..=5 => println!("x is between 2 and 5: {}", n), // @绑定
        _ => println!("x is something else"),
    }
}