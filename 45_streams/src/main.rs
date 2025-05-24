/// stream
/// 消息传递中的异步的revc方法会会随着时间产生一系列的消息 这称为stream
/// stream是一个异步的迭代器
/// 可以从iterator中创建stream
use trpl::{stream_from_iter,StreamExt};
fn main() {
    trpl::run(async {
        let v=vec![1, 2, 3, 4, 5];
        let mut stream=stream_from_iter(v.into_iter());
        while let Some(item)=stream.next().await {
            println!("item: {}",item);
        }
    })

    
}