/*
必须穷尽
match a{  
  a1:xxxx
  a2:sads
  other=>asdaads // other可换名或者 _
}
#[test]
fn match_owner(){
    let opt = Some(
        String::from("alabama")
    );
    match opt {
        Some(i) => { // i获得所有权
            println!("{}", i);
        },
        None =>{
            println!("None");
        }
    }
    println!("{:?}",opt);// 失去权限了
    
}
修改 match &opt
if let 的使用
#[test]
fn if_let(){
    let opt = Some(3);
    if let Some(i) = opt {
        println!("Some number is {}", i);
    }
}
 */
use std::fmt::Debug;

mod option_match;


#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}
fn value_in_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
fn main() {
    let value=value_in_cents(Coin::Quarter(UsState::Alabama));
    let five:Option<i32>= Some(value);
    let six=option_match::plus_one(five);
    match six {
        None => println!("None"),
        Some(i) => {
            println!("Some number is {}", i);
        }
    }
}
#[test]
fn match_owner(){
    let opt = Some(
        String::from("alabama")
    );
    match &opt {
        Some(i) => { // i获得所有权
            println!("{}", i);
        },
        None =>{
            println!("None");
        }
    }
    println!("{:?}",opt);// 失去权限了
    
}
#[test]
fn if_let(){
    let opt = Some(3);
    if let Some(i) = opt {
        println!("Some number is {}", i);
    }
}