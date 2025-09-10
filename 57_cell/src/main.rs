use std::cell::Cell;

fn main() {
    // 实现了Copy这个trait
    let ce=Cell::new(5);
    println!("get方法：{}",ce.get());
    // 10代替里面的5
    let a=ce.replace(10);
    println!("ce.replace(10):{}",a);
    // 对于没有实现Copy的数据
    let cell_string=Cell::new(String::from("Hello"));
    // take
    let res=cell_string.take();
    println!("cell::take={}",res);
    // 设置default
    let res=cell_string.take();
    println!("ce.take():{}",res);

    // into_inner,ce被消耗掉了
    let ten=ce.into_inner();
    println!("ce.into_inner()={}",ten);
}