/*
<T> 声明了一个泛型类型参数 T
添加特性约束，使代码能够编译通过
// 添加PartialOrd和Copy特性约束
1、在函数定义中使用泛型
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn lib() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("最大的数字是 {}", result);
    
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("最大的字符是 {}", result);
}
2、在结构体中使用泛型
struct Point<T> {
    x: T,
    y: T,
}
struct Point<T, U> {
    x: T,
    y: U,
}
3、在枚举中使用泛型
enum Option<T> {
    Some(T),
    None,
}
4、在方法中使用泛型
struct Point<T> {
    x: T,
    y: T
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
不能为泛型类型和具体类型实现相同的方法
使用泛型不会导致程序变慢
泛型代码进行编译时进行单态化，这意味着编译器会为每一个具体使用的类型生成特定的代码
 */
use std::fmt::{format, Display};

struct Point<T> {
    x: T,
    y: T
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}
fn print_slice<T:Display>(v:&[T]){
    for x in v{
        println!("{}", x);
    }
}
#[test]
fn test_main() {
    print_slice(&[1,2,3]);
    
}