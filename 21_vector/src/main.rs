/*
创建vector
    let v:Vec<i32> =Vec::new();
    let v1=vec![1,2,3,4,5];
添加vector
    let mut v=Vec::new();
    v.push(1);
    v.push(2);
读取vector
    let v=vec![1,2,3,4,5];
    let third=&v[2];// & []
    println!("&v[2]返回的是{:?}", third);
    let fourth=v.get(2);// get 返回option
    println!("get返回{:?}", fourth);
    if let Some(a)=fourth{
        println!("获取get返回{:?}", a);
    }
    索引越界
&v[100] ——panic
get(100)——None
遍历vector
    let v=vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
    for n_ref in &v{
        let  n_1=*n_ref+1;
        println!("n_1={}",n_ref);
    }
    
    
     let mut v=vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
    for n_ref in &mut v{
        *n_ref+=1;
    }
    for n in &v{
        println!("{}",n);
    }
迭代器遍历vector
fn dup_in_place(vec: &mut Vec<i32>) {
    for item in vec.iter() {// 不可变
        vec.push(*item+1);// 可变的引用
    }
}
 */
fn main() {
    
}


// #[test]
// fn test1(){
//     let mut v=Vec::new();
//     let s=String::from("hello");
//     v.push(s);
//     v[0].push_str(" world");
//     println!("{}",s);
//     println!("{}",v[0]);
// }
// #[test]
// fn test2(){
//     let v=vec![String::from("hello ")];
//     let mut s=v[0]; //不可变引用赋值给一个可变变量
//     s.push_str(" world");
//     println!("{}",s);
// }
#[test]
fn test3(){
    let mut v=vec![1,2,3];
    let mut v2:Vec<&mut i32> =Vec::new();
    for x in &mut v{
        v2.push(x);
    }
    *v2[0]=5;
    let a=*v2[0];
    let b=v[0];
    println!("a={}",a);
    println!("b={}",b);
    
}
