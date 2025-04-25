/*
生命周期:确保引用在所需的时间内有效,引用有效的范围
每个引用都有生命周期
大多数情况下,生命周期是隐含的,不需要指定
防止悬垂引用
fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("r: {}", r);
    
}
  --> src\lib:11:13
   |
10 |         let x = 5;
   |             - binding `x` declared here
11 |         r = &x;
   |             ^^ borrowed value does not live long enough
12 |     }
   |     - `x` dropped here while still borrowed
13 |     println!("r: {}", r);
   |                       - borrow later used here4
   借用检查器 Borrow Checker 确保数据存活长与引用
   
     --> src\lib:31:27
   |
31 | fn logest(x:&str,y:&str)->&str{
   |             ----   ----   ^ expected named lifetime parameter
   |
   
   
   fn logest(x:&str,y:&str)->&str{
    if x.len()>y.len(){
        x
    }else{
        y
    }
}
   = help: this function's return type contains a borrowed value,
    but the signature does not say whether it is borrowed from `x` or `y`
    
    语法
    1、必须以'开头,生命周期参数名通常全小写,非常短
    2、紧跟在&之后,引用的生命周期参数，用空格分隔
    &i32
    &'a i32
    &'a mut i32
    
    fn logest<'a>(x:&'a str,y:&'a str)->&'a str{
    if x.len()>y.len(){
        x
    }else{
        y
    }
    x y生命周期短的返回
        let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = logest(string1.as_str(),string2.as_str());
    }
    println!("The longest string is {}", result);
    result的生命周期是string2的生命周期，报错了
    结构体中的生命周期
    struct ImportantExcerpt<'a> {
    part: &'a str,
    }
    当part的引用存在时，ImportantExcerpt实例也必须存在
    如果part的引用不存在，ImportantExcerpt实例也不应该存在
    输入生命周期：在函数，方法从参数上的生命周期
    输出生命周期：在返回值上的生命周期
三条规则（没有显式指定生命周期参数）    
1、每个引用的参数都有自己的生命周期参数
2、如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数
3、如果方法有多个输入生命周期参数，不过其中一个是&self或&mut self，那么self的生命周期被赋予所有输出生命周期参数
方法定义中的生命周期注解
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
 静态生命周期
 'static 生命周期，整个程序运行期间都有效
 struct Foo<'a> {
    x: &'a i32,
}
fn baz<'a>(f:&'a Foo<'a>)->&'a i32{
    f.x
}
 */
fn main() {
    let string1 = String::from("long string is long");
    
}
