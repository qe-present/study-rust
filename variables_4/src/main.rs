// 常量和变量的区别
// 常量是不可变的，而变量是可变的
// 常量使用const关键字声明，而变量使用let关键字声明
// 常量 不可以使用mut修饰，而变量可以使用mut修饰
// 常量可以在任何作用域中声明，包括全局作用域，而变量只能在声明它的作用域中使用
// 常量只能被设置为常量表达式，而变量可以根据需要改变
// 常量的类型必须标注，而变量的类型可以省略，编译器会根据变量的值推断类型
// 常量使用大写字母和下划线命名，而变量使用小写字母和下划线命名 如MAX_POINTS
const MAX_POINTS: u32 = 100_000;
fn main() {
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
    println!("Hello, world!");
    //let x = 5; //变量是不可变的
    let mut x = 5; //mut 修饰的变量是可变的
    let mut x = x+1; // shadowing 隐藏
    println!("The value of x is: {}", x);
    x=7;
    println!("The value of x is: {}", x); //  没有mut=> cannot assign twice to immutable variable
    let x = x*2;
    println!("The value of x is: {}", x);
    let spaces = "       ";
    let spaces = spaces.len(); //shadowing
    println!("The value of spaces is: {}", spaces);
    // let mut spaces = "       ";
    // spaces = spaces.len(); // expected `&str`, found `usize`
    // println!("The value of spaces is: {}", spaces); 类型不匹配
}
