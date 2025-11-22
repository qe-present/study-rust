#[cfg(test)]
mod test {
    use crossbeam::scope;
    use std::sync::atomic::{AtomicUsize,Ordering};
    use std::thread; // 需要引入
    use std::time::Duration; // 需要引入
    #[test]
    fn test_atomic_num() {
        let data = AtomicUsize::new(0);
        let _ = scope(|scope| {
            for _ in 0..10 {
                scope.spawn(|_| {
                    for _ in 0..100 {
                        // 睡眠
                        thread::sleep(Duration::from_millis(20));
                        data.fetch_add(1,Ordering::Relaxed);
                    }
                });
            }
            loop {
                let n=data.load(Ordering::Relaxed);
                if n==1000{
                    break;
                }
                println!("progress {n}/1000 done");
                thread::sleep(Duration::from_secs(1));
            }
        });
        println!("all done");

    }
    #[test]
    fn test_exchange() {
        fn incr(a:&AtomicUsize){
            let mut current = a.load(Ordering::Relaxed);
            loop {
                let new=current+1;
                match a.compare_exchange(current,new,Ordering::Relaxed,Ordering::Relaxed){
                    Ok(_) => return,
                    Err(v) => {
                        println!("Err:{}",v);
                        current=v;
                    },
                }
            }

        }
        // CAS loop (Compare-And-Swap 循环)
        let data = AtomicUsize::new(0);
        let _ = scope(|scope| {
            for _ in 0..1000 {
                scope.spawn(|_| {
                    incr(&data);
                });
            }
        });
        println!("data {}",data.load(Ordering::Relaxed));

    }
}
