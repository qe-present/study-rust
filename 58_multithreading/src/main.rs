use std::panic;

// Mutex 互斥锁，一种用于保护共享数据的互斥原语
// 原语(primitive):最基本、不可再分解的操作或机制
///
/// - 最常用于在线程间分享数据的工具
/// - 只允许对数据独占（exclusive）访问
/// - 临时阻塞同一时刻想要访问数据的其他线程
/// 两种状态 locked，unlocked
/// RwLock 同一时刻有多个读取者和一个写入者,其中泛型是Send+Sync
fn main() {}
#[test]
fn test_mutex() {
    use crossbeam::scope;
    use std::sync::{Arc, Mutex};
    let numbers = Arc::new(Mutex::new(Vec::new()));
    scope(|s| {
        for i in 0..20 {
            let numbers_clone = Arc::clone(&numbers);
            s.spawn(move |_| {
                let mut lock = numbers_clone.lock().unwrap();
                lock.push(i);
            });
        }
    })
        .unwrap();
    let lock = numbers.lock().unwrap();
    assert_eq!(lock.len(), 20);
}
#[test]
fn test_poisoned() {
    use crossbeam::scope;
    use std::sync::{Arc, Mutex};
    let data = Arc::new(Mutex::new(0));
    let scope_result = scope(|s| {
        let data_clone = Arc::clone(&data);
        s.spawn(move |_| {
            let mut lock = data_clone.lock().unwrap();
            *lock += 1; // 修改数据
            panic!("panic!"); // 持有锁时 panic，导致锁被毒化
        });
    });
    assert!(
        scope_result.is_err(),
        "scope should return Err due to panic in thread"
    );
    match data.lock() {
        Ok(_) => panic!("lock is poisoned"),
        Err(poisoned) => {
            let mut guard = poisoned.into_inner();
            *guard += 1;
            assert_eq!(*guard, 2);
        }
    }
}
#[test]
fn test_relock() {
    use crossbeam::scope;
    use std::sync::Arc;
    use std::sync::RwLock;
    let counter = Arc::new(RwLock::new(0));
    let _=scope(|s| {
        for i in 0..20{
            let counter_clone = Arc::clone(&counter);
            s.spawn(move |_| {
                let lock = counter_clone.read().unwrap();
                println!("{}-{:?}",i, lock);
            });
        }
    });
    assert_eq!(*counter.read().unwrap(), 0);
    let _=scope(|s|{
        let data_clone = Arc::clone(&counter);
        s.spawn(move |_| {
            let mut lock = data_clone.write().unwrap();
            *lock+=1;
        });
    });
    assert_eq!(*counter.read().unwrap(), 1);
}