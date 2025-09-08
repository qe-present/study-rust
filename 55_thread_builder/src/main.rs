use std::thread;

fn main(){
    let handle=thread::Builder::new()
        .name("hello thread".into())
        .stack_size(4*1024*1024)
        .spawn(move || {
            println!("{}",thread::current().name().unwrap());
        })
        .unwrap();
    handle.join().unwrap();
}