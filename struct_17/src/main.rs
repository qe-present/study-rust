mod tuple_struct;
mod derived_trait;

use tuple_struct::Color;
use derived_trait::Rectangle;
fn main() {
    let mut user1=User{
        email: String::from("hello"),
        username: String::from("world"),
        active: true,
        sign_in_count: 1,
    };
    user1.email=String::from("hello@world");
    let c1=Color::new(0, 0, 0);
    let rect1=Rectangle{
        width:100,
        height:100
    };
    println!("rect is {rect1:#?}");
    let rect2=Rectangle{
        width:120,
        height:200
    };
    
    
}
struct User {
    active:bool,
    username:String,
    email:String,
    sign_in_count:u64,
}
// 创建struct
fn build_user(email:String, username:String) -> User {
    User {
        email,
        username,
        active:true,
        sign_in_count:1,
    }
}
// 创建struct
fn build_user2(user: User) -> User {
    User {
        email:String::from("hello@world1"),
        ..user
    }
}
struct Point{
    x:i64,
    y:i64,
}
impl Point{
    fn get_x(&mut self)->&mut i64{
        &mut self.x
    }
}
// 结构体的字段是独立的。对一个字段的引用不会影响对其他字段的引用
#[test]
fn test1() {
    let mut p=Point{x:0,y:0};
    let x=&mut p.x;
    let y=&mut p.y;
    *x+=1;
    *y+=1;
    println!("x={}",p.x);
    println!("y={}",p.y);
    
}
#[test]
fn test2() {
    let mut a=Point{x:0,y:0};
    a.x+=1;
    let b=Point{
        y:1,
        ..a
    };
    a.x+=1;
    println!("x={}",a.x);
}
#[test]
fn test3() { 
    let mut a=Point{x:0,y:0};
    let x=a.get_x(); // a失去所有权限，可变引用，也被称为独占引用，&mut self=> &mut a
    *x+=1;
    println!("x={} y={}",*x,a.y);
}