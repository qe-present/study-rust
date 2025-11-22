use std::sync::atomic::{AtomicU32, Ordering};

fn main() {
    // 创建原子变量
    let atomic_num = AtomicU32::new(100);

    // ==================== load 方法 ====================
    // 加载当前值
    let value = atomic_num.load(Ordering::SeqCst);
    println!("current value: {}", value);

    // ==================== store 方法 ====================
    // 存储新值
    atomic_num.store(200, Ordering::SeqCst);
    println!("after store(200): {}", atomic_num.load(Ordering::SeqCst));

    // 恢复初始值
    atomic_num.store(100, Ordering::SeqCst);

    // ==================== 常用原子操作方法 ====================

    // 1. fetch_add - 原子加,返回旧值
    let old_value = atomic_num.fetch_add(50, Ordering::SeqCst);
    println!("\nfetch_add(50)");
    println!("  返回的旧值: {}", old_value);
    println!("  当前值: {}", atomic_num.load(Ordering::SeqCst));

    // 2. fetch_sub - 原子减,返回旧值
    let old_value = atomic_num.fetch_sub(30, Ordering::SeqCst);
    println!("\nfetch_sub(30)");
    println!("  返回的旧值: {}", old_value);
    println!("  当前值: {}", atomic_num.load(Ordering::SeqCst));

    // 3. fetch_and - 原子按位与,返回旧值
    let old_value = atomic_num.fetch_and(0b11110000, Ordering::SeqCst);
    println!("\nfetch_and(0b11110000)");
    println!("  返回的旧值: {}", old_value);
    println!("  当前值: {}", atomic_num.load(Ordering::SeqCst));

    // 恢复原值
    atomic_num.store(100, Ordering::SeqCst);

    // 4. fetch_or - 原子按位或,返回旧值
    let old_value = atomic_num.fetch_or(0b00001111, Ordering::SeqCst);
    println!("\nfetch_or(0b00001111)");
    println!("  返回的旧值: {}", format!("{:08b}", old_value));
    println!("  当前值: {:08b}", atomic_num.load(Ordering::SeqCst));

    // 恢复原值
    atomic_num.store(100, Ordering::SeqCst);

    // 5. fetch_xor - 原子按位异或,返回旧值
    let old_value = atomic_num.fetch_xor(0b00001111, Ordering::SeqCst);
    println!("\nfetch_xor(0b00001111)");
    println!("  返回的旧值: {}", format!("{:08b}", old_value));
    println!("  当前值: {:08b}", atomic_num.load(Ordering::SeqCst));

    // 恢复原值
    atomic_num.store(100, Ordering::SeqCst);

    // 6. fetch_update - CAS 循环,原子更新值
    let result = atomic_num.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| {
        // 如果当前值是100,就返回新值150
        if x == 100 {
            Some(150)
        } else {
            None // 更新失败
        }
    });
    println!("\nfetch_update");
    match result {
        Ok(old_value) => println!("  更新成功,旧值: {},新值: {}", old_value, atomic_num.load(Ordering::SeqCst)),
        Err(current_value) => println!("  更新失败,当前值: {}", current_value),
    }

    // 7. compare_exchange - 比较并交换（强CAS）
    let current = atomic_num.load(Ordering::SeqCst);
    let result = atomic_num.compare_exchange(
        current,        // 期望的旧值
        current + 50,   // 新值
        Ordering::SeqCst,
        Ordering::SeqCst,
    );
    println!("\ncompare_and_exchange");
    match result {
        Ok(old_value) => println!("  交换成功,旧值: {},新值: {}", old_value, atomic_num.load(Ordering::SeqCst)),
        Err(current_value) => println!("  交换失败,当前值: {}", current_value),
    }

    // 8. swap - 原子交换,返回旧值
    let old_value = atomic_num.swap(300, Ordering::SeqCst);
    println!("\nswap(300)");
    println!("  返回的旧值: {}", old_value);
    println!("  当前值: {}", atomic_num.load(Ordering::SeqCst));
}
