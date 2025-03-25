use std::io; //prelude
use rand::Rng; //trait
use std::cmp::Ordering;

fn main() {
    println!("猜数!");


    let secret_number = rand::thread_rng().gen_range(1..101); //i32 u32 i 64 默认是i32
    loop {
        println!("猜测一个数");
        // 变量是不可变的，mut 修饰的变量是可变的
        let mut guess = String::new();
        // & 表示引用，引用是一种借用，不会拥有所有权，也是不可变的，需要加上mut修饰
        // read_line 方法返回一个io::Result类型的值，io::Result类型是一个枚举类型，有两个值，Ok和Err OK表示成功，Err表示失败
        // expect 方法是io::Result类型的一个方法，如果io::Result类型的值是Err，expect方法会导致程序崩溃，并显示expect方法中的信息
        io::stdin().read_line(&mut guess).expect("无法读取行");
        // trim 方法去掉字符串首尾的空白字符 parse方法将字符串转换为数字 expect方法处理错误
        // shadow 隐藏
        //let guess: u32 = guess.trim().parse().expect("请输入一个数字");
        let guess: u32 =match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        // {} 是占位符，可以在字符串中插入变量
        println!("你猜测的数是: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {
                println!("你赢了");
                break;
            },
        }
    }
}
