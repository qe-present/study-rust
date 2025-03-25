/*
可变引用提供对数据"唯一的"且“非拥有的”访问
不可变引用(共享引用)：只读的
可变引用(独占引用)：在不移动数据的情况下，临时提供可变访问

 */

fn main() {
    let mut v=vec![1,2,3,4,5];
    let num=&mut v[2]; // &mut 可变引用，num不可变，*num可变
    let num2=& *num;
    println!("{},{}",num,num2);
    
}
