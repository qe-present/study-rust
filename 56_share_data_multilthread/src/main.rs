use std::sync::Arc;
use std::thread;


fn main() {
    let data=Arc::new(Box::new([1,2,3,4,5]));
    let mut handles=Vec::new();
    for i in 0..10000 {
        let local_data=data.clone();
        let h=thread::spawn(move || {
            println!("{local_data:?}");
        });
        handles.push(h);
    }
    handles.into_iter().for_each(|h|{
        h.join().unwrap();
    });

}