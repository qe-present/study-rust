/*
可恢复的错误
不可恢复的错误-bug
两种导致panic的方式
1、代码中的某些行为
2、显示调用panic的宏
Backtrack
 -- 到达某个点之前所调用的所有函数的列表
设置环境变量
RUST_BACKTRACE="任何不为0的值" 必须在debug模式下  RUST_BACKTRACE=full


--可恢复的错误
enum Result<T,E>{
        Ok(T)
        Err(E) 
}
use std::fs::File;
use std::io::ErrorKind;

fn main.rs() {
    let result = File::open("./hello.txt");
    let files = match result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound =>match File::create("./hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Failed to create file") 
            },
            other_error => panic!("{:?}", other_error)
        }
    };
}

出现错误使程序产生恐慌的常用快捷方式
unwrap(),用于提取Option或Result类型内部的值 如果是None或者Err,程序panic并终止
expect(),与unwrap类型，允许提供一个自定义的panic消息
    let result = File::open("./hello1.txt").unwrap();
        let result = File::open("./hello1.txt").expect("Failed to open hello1.txt");
        
        
传播错误
将错误返回，由调用该函数的代码来决定任何处理错误
fn read_username_from_file()->Result<String,io::Error>{
    let file = File::open("username.txt");
    let mut username_file=match file {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut username=String::new();
    match username_file.read_to_string(&mut username){
        Ok(_)=>Ok(username),
        Err(e) => Err(e),
    }
}
?运算符
fn read_username_from_file()->Result<String,io::Error>{
    let mut username=String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
可以链式调用
fn read_username_from_file()->Result<String,io::Error>{
    fs::read_to_string("hello.txt")
}


-- 如果成功，解包Ok
-- 如果失败，返回Err
避免if let 或match
需要函数的返回类型和?起作用的返回类型一致
需要返回Result Option fromResidual

from函数
把值从一个类型转化位另一个类型
-- 定义在std的From trait
use std::fs;
use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};
use std::num::ParseIntError;
#[derive(Debug)]
pub enum MyError{
    Io(io::Error),
    ParseInt(ParseIntError),
    Other(String)
}
impl From<io::Error> for MyError {
    fn from(err: io::Error) -> MyError {
        MyError::Io(err)
    }
}
impl From<ParseIntError> for MyError {
    fn from(err: ParseIntError) -> MyError {
        MyError::ParseInt(err)
    }
}

fn read_username_from_file()->Result<String,MyError>{
    let mut username = String::new();
    let file=File::open("hello.txt")?.read_to_string(&mut username)?;
    let num:i32="111".parse()?;
    Ok(username)
}

main函数
可以返回实现了trait std::process::Termination这个trait的类型


#[test]
fn read_username_file() ->Option<String>{
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username);
    Some(username) 
    //在返回 `Option` 的函数中，`?` 运算符只能用于 `Option`，而不能用于 `Result`。 [E0277]
}
 */

fn main() {

}

