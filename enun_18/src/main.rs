/*
enum
enum IpAddrKind{
    V4,
    V6
}
    //使用enum
    let four=IpAddrKind::V4;
    let six=IpAddrKind::V6;
enum IpAddrKind{
    V4(String),
    V6(String)
}
    let four=IpAddrKind::V4(String::from("127.0.0.1"));
 enum IpAddrKind{
    V4(u8, u8, u8, u8),
    V6(String)
}
    let four=IpAddrKind::V4(127, 0, 0, 1);
可以使用impl定义方法
enum Option<T>{
  None,
  Some(T)
}
表示某个值可能存在或不存在
    let some_number = Some(5);
    let some_string = Some(String::from("hello"));
    let absent_number:Option<i32> = None;
*/
mod study_enum;

fn main() {
    let x:i8=5;
    let y:Option<i8>=Some(5);
    let sum=x+y.unwrap_or_else(||0);
}