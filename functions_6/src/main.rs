// 用return 或者函数体中最后一个表达式的值
fn main() {
    another_func(12312);
    another_func2();
    let a=another_func3();
    println!("{}",a);
}
fn another_func(x:i32) {
    println!("_{}_",x);
}
fn another_func2() {
    let y={
        let x = 123;
        x+1
    };
    println!("{}",y);
}
fn another_func3()->i32 {
    let x = 123123;
    x+1
}