/*
创建string
    let s=String::new();
    let data="asds";
    let d_str=data.to_string();
    let s="adadsas".to_string();
    let s=String::from("asdsas");
push String
    let mut s=String::from("asdasds");
    s.push_str("sfwasd");
    println!("{}",s);
    let mut s=String::from("asdasd");
    s.push('a');
    println!("{}",s);
连接String
    let s1=String::from("hello");
    let s2=String::from("world");
    let s3=s1+&s2; // add(self,&str) s1就失去所有权限
    println!("{}",s3);
        let s1 = String::from("Hello, world!");
    let s2 = String::from("Hello, world!");
    let s3 = String::from("Hello, world!");
    let s=format!("{} {} {}",s1,s2,s3);
    println!("{}",s);
 String 索引访问
 不可以通过索引访问string
 三种与String相关的视角 字节 标量值，字形簇
 string 进行切片
     let hello=String::from("Hello");
    let s=&hello[0..3];
    println!("{}",s);
 String 进行遍历
     let s=String::from("hello");
    for i in s.chars() { 单个Unicode标量值
        println!("{}", i);
    }
    for i in s.bytes() { 原始字节
        println!("{}", i);
    }
 */
fn main() {
    
}
