use std::thread;
use std::time::Duration;

fn main() {
    // let a = String::from("hello");
    // thread::scope(|s| {
    //     let b = String::from("hello");
    //     for i in 0..5 {
    //         s.spawn(move|| {
    //             thread::sleep(Duration::from_secs(1));
    //             println!("thread #{}", i);
    //         });
    //     }
    // })
    const CHUNK_SIZE: usize = 10;
    let numbers:Vec<i32>=(1..10000).collect();
    let chunks=numbers.chunks(CHUNK_SIZE);
    let sum=thread::scope(|s|{
        let mut handles=Vec::new();
        for chunk in chunks {
            let h=s.spawn(move ||{
                chunk.iter().sum::<i32>()
            });
            handles.push(h);
        };
        handles.into_iter().map(|h|{
            h.join().unwrap()
        }).sum::<i32>()
    });
    println!("{:?}",sum);
}
