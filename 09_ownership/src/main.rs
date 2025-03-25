// 引用是没有所有权的指针
fn main() {
    let m1=String::from("hello");
    let m2=String::from("world");
    println!("m1的地址:{:p}",&m1);
    greet(&m1,&m2);
    println!("{}",m1);
    println!("{}",m2);
}
fn greet(g1:&String, g2:&String) {
    println!("{} {}", g1, g2);
    let address_in_g1=g1 as *const String;
    println!("{g1}");
    println!("g1存的内容：{:p}",address_in_g1);
    println!("g1的地址:{:p}",g1);
}
#[test]
fn test_good() {
    let m1=String::from("hello");
    println!("{}",m1);
    println!("{}",&m1)
}