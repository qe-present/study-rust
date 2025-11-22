#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use crossbeam::scope;
    use std::thread;
    use std::time::Duration;
    #[test]
    fn it_works() {
        let flag=Arc::new(AtomicBool::new(false));
        let flag2=Arc::clone(&flag);
        let _ = scope(|scope| {
            let handle = scope.spawn(move |_| {
                while !flag2.load(Ordering::Relaxed) {
                    println!("线程等待中...");
                    thread::park(); // 挂起
                    println!("线程被唤醒!");
                }
                println!("条件满足，退出循环");
            });

            thread::sleep(Duration::from_secs(2));
            let handle2 = handle.thread().clone();
            // 唤醒线程
            scope.spawn(move |_| {
                println!("start");
                flag.store(true, Ordering::Relaxed);
                handle2.unpark();
            });
        });


    }
}
