/// 使用迭代器和闭包等高级特性，提供了更高层次的代码抽象，保持了代码的简洁性和可读性。
///
///
///
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    // 使用迭代器和闭包
    let new_v= v.iter()
        .map(|x| x * 2)
        .collect::<Vec<i32>>();
    dbg!(new_v);

}
