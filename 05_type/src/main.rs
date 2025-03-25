// rust 是静态编译语言 在编译时就需要确定类型
/*
—基于使用的值，编译器通常能够推断出它的具体类型
— 但如果可能的类型比较多（例如把String 转为整数parse的方法）就必须知道添加类型的标注，否则编译会错误
 */
// 标量类型
/*
— 整数类型

— 浮点类型
— 布尔类型
— 字符类型

tuple 元组 固定长度 包含不同类型的元素
array 数组 固定长度 元素类型相同

 */
fn main() {
    let num_a=45;
    let num_b:u32=46;
    let num_c:u8=255;
    let num_d:usize=46;
    let num_e=0xff;
    let num_f:u8=b'A';
    
    let x=2.0;
    let y:f32=4.0;
    
    let t=true;
    let f:bool=false;
    
    let c='z';
    let z='A';
    let chinese='我';
    let emoji='1';
    
    let tup=(100,'a',false);
    let tup:(i32,f64,u8)=(500,6.5,1);
    let (x,y,z) = tup;
    println!("x:{},y:{},z:{}",x,y,z);
    
    let arr=[1,2,3,4,5,6,7,8,9,10,11];
    let b:[i32;5]=[1,2,3,4,5];
    let c=[3;5];
    println!("arr[0]:{}",arr[0]);
    
    
    




}
