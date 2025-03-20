/*
Rust 通过"借用检查器"确保引用的安全性
变量对数据据有三种权限
 1、读(R):数据可以被复制带另一个位置
 2、写(W):数据可以被修改
 3、拥有(O):数据可以被移动或者释放
 这些权限在运行时并不存在，仅在编译器内部存在
 默认情况下，变量对器数据具有读/拥有的权限
 变量被注解为let mut,则还具有写的权限
 关键 引用可以临时移除这些权限
 */
fn main() {
    let x=0;
    let mut x_ref=&x;
    let y=1;
    //*x_ref+=1;// *x_ref相当于x，x是不可变的
    x_ref=&y; //
    println!("x_ref={}",x_ref);
}
#[test]
fn test_own(){
    let mut v=vec![1,2,3]; // R W O
    let num=&v[2]; 
    println!("{}",num);
    println!("{}",*num);
    println!("{}",v[2]);
}
