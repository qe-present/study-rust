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

 */
fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess: {}", guess);

}
