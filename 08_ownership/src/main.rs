// 如果一个变量拥有一个box。当Rust释放该变量的frame时，Rust会释放该box的堆内存
// clone避免移动-所以权的专业
fn main() {
    let first=String::from("hello");
    let full=add_suffix(first.clone());
    println!("{}",first);
    println!("{}",full);
}
fn add_suffix(mut name: String) -> String {
    name.push_str(" world");
    name
}
#[test]
fn test_good() {
    let s=String::from("hello");
    let s2;
    let b=false;
    if b{
        s2=s.clone();
    }
    println!("{}",s);
}
