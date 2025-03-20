/*
修改所有权的常见错误
1、返回引用指向Stack
fn return_a_string1() -> &String {
    let s=String::from("hello");
    return &s;
} 返回引用，但是s被销毁
 2、没有足够的权限
 fn stringify_name_with_title(name:&Vec<String>)->String {
    name.push(String::from("hello"));
    let full=name.join(" ");
    full
}
3 别名和可变性
fn add_big_strings(dst:&mut Vec<String>,src:&[String]){
    let largest=dst.iter().max_by_key(|x|x.len()).unwrap();
    for s in src {
        if(s.len() > largest.len()){
            dst.push(s.clone());
        }
    }
}
4、 从一个集合拷贝一个元素
fn test_4(){
    let v=vec![0,1,2];
    let v_ref=&v[0];
    let n=*v_ref;
    let v_str=vec![String::from("hello")];
    let v_str_ref=&v_str[0];
    let n_str=*v_str_ref; // clone解决

}
5 修改tuple不同字段的问题
fn  test_5(){
    let mut name=(
        String::from("John"),
        String::from("Doe")
    );
    let first=get_first(&mut name);
    name.1.push_str("1123"); //  ^^^^^^ second mutable borrow occurs here
    println!("{}",first);
}
fn get_first(name: &mut(String,String))->&String{
    &name.0
}
6、可变引用和不可变引用的问题
fn  test_6(){
    let mut a=vec![1,2,3];
    let b=&mut a[0];
    // a失去了所有的权限
    *b+=1;
    let y=&a[2]; // immutable borrow occurs here
    *b += *y;
    println!("{:?}",a);
}
*/



// use std::rc::Rc;
fn main() {
    test_6()
}
// fn return_a_string1() -> &String {
//     let s=String::from("hello");
//     return &s;
// }
// 返回s的引用。 s被去除报错
// fn return_a_string2() -> String {
//     let s=String::from("hello");
//     return s;
// }
// fn return_a_string3() -> &'static str {
//     "world"
// }
// fn return_a_string4() -> Rc<String> {
//     let s=Rc::new(String::from("world"));
//     Rc::clone(&s)
// }
// fn return_a_string5(output:&mut String) {
//     output.replace_range(..,"hello world");// 从0开始，到最后一个字符
// 
// }

// fn  test_6(){
//     let mut a=vec![1,2,3];
//     let b=&mut a[0];
//     *b+=1;
//     let y=&a[2]; // immutable borrow occurs here
//     *b += *y;
//     println!("{:?}",a);
// }