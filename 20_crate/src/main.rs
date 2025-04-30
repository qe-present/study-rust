/*
crate 是组织和共享代码的基本构建块
binary crate:可执行的，需要有main函数
library crate:没有main函数,无法执行。定义一些功能，可共享使用

crate root
binary crate :src/lib
library crate: src/main

package 由1个或多个crates组成
1、多个binary crates
2、最多1个library cates
3、但至少由一个crate
创建library :cargo new project_name --lib


Module 模块
声明 mod name
引用 use

1、mod model {
// inline
}
2、model.rs
3、model/mod.rs

mod m1{
    pub mod m2{  //m2 没有pub，则是私有的
        pub fn method1(){
            println!("Hello, world!");
        }
        
    }
    
}
mod x1{
    mod x2{
        fn method2(){
            println!("Hello, world!!!");
            // super:: 来到x1
            //  super::super::  来到crate 
            // 相对路径 super self
            super::super::m1::m2::method1();

        }

    }

}
所有的东西默认对父模块是私有的
struct 要为字段和本身加上pub
pub struct HousePrice{
    pub price: u32,
    pub area:String,
    pub bed_rooms:u32,
    pub main_road:YesNo
}
as 关键字 给引用器别名
// pub use
 */
use crate::models::enums::YesNo;
use crate::models::structs::HousePrice;

mod models;
fn main() {
    let y=YesNo::Yes;
    let house_price=HousePrice{
        price:1000,
        area:String::from("hello"),
        bed_rooms:100,
        main_road:YesNo::Yes
    };
    
}
