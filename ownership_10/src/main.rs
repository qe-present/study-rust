// * 解引用获得数据
//关于隐示解引用
//1 x.abs()
//2 隐式解引用支持多层
//3 反向亦可
fn main() {
    let mut x:Box<i32>=Box::new(1);// x是指针，类型是BOX
    let a:i32=*x;
    println!("{}",a);
    *x+=1;
    let r1=&x;// 两次引用
    println!("{}",**r1);
    let r2=&*x;// & *抵消
    println!("{}",*r2);
}
#[test] //解引用
fn test_solve_quote() {
    let x:Box<i32>=Box::new(-1);
    let x_abs1=i32::abs(*x); //显示解引用
    let x_abs2=x.abs(); // 隐示解引用
    assert_eq!(x_abs1, x_abs2);
}
#[test]
fn test_two_solve_quote() {
    let r:&Box<i32>=&Box::new(-1);
    let r_abs1=i32::abs(**r); // 显示解引用两次
    let r_abs2=r.abs(); //隐示解引用两次
    assert_eq!(r_abs1, r_abs2);
}
#[test]
fn test_solve_quote2() {
    let s=String::from("hello");
    let s_len1=str::len(&s);  // &s 显示引用
    let s_len2=s.len();   // 隐式引用
    assert_eq!(s_len1,s_len2);
}