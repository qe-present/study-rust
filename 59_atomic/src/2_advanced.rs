use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

// ============================================
// 示例 1: 自旋锁（Spinlock）实现
// ============================================
#[derive(Debug)]
struct Spinlock {
    locked: AtomicBool,
}

impl Spinlock {
    fn new() -> Self {
        Spinlock {
            locked: AtomicBool::new(false),
        }
    }

    // 获取锁（自旋等待）
    fn lock(&self) {
        while self
            .locked
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            // 自旋等待时可以使用pause指令提示CPU
            std::hint::spin_loop();
        }
    }

    // 释放锁
    fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }

    // 尝试获取锁（非阻塞）
    fn try_lock(&self) -> bool {
        self.locked
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
    }
}

// 使用自旋锁保护共享数据
fn spinlock_example() {
    println!("=== 自旋锁示例 ===");
    let lock = Arc::new(Spinlock::new());
    let mut handles = vec![];

    let shared_data = Arc::new(AtomicU32::new(0));

    for i in 0..5 {
        let lock_clone = Arc::clone(&lock);
        let data_clone = Arc::clone(&shared_data);
        let handle = thread::spawn(move || {
            println!("线程 {}: 尝试获取锁", i);
            lock_clone.lock();
            println!("线程 {}: 获得锁", i);

            // 在锁保护下修改数据
            let old = data_clone.fetch_add(10, Ordering::SeqCst);
            println!("线程 {}: 修改数据从 {} 到 {}", i, old, old + 10);

            thread::sleep(Duration::from_millis(100));

            lock_clone.unlock();
            println!("线程 {}: 释放锁", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("最终值: {}", shared_data.load(Ordering::SeqCst));
    println!();
}

// ============================================
// 示例 2: 无锁计数器（带延迟更新优化）
// ============================================
struct DelayedCounter {
    // 本地计数器（避免频繁修改全局计数器）
    local_counts: Vec<AtomicU32>,
    // 全局计数器（定期同步）
    global_count: AtomicU32,
}

impl DelayedCounter {
    fn new(num_threads: usize) -> Self {
        DelayedCounter {
            local_counts: (0..num_threads)
                .map(|_| AtomicU32::new(0))
                .collect(),
            global_count: AtomicU32::new(0),
        }
    }

    // 增加本地计数
    fn increment(&self, thread_id: usize) {
        self.local_counts[thread_id].fetch_add(1, Ordering::Relaxed);
    }

    // 同步到全局计数器（批量更新）
    fn sync(&self, thread_id: usize) {
        let local = self.local_counts[thread_id].swap(0, Ordering::Acquire);
        if local > 0 {
            self.global_count.fetch_add(local, Ordering::Release);
        }
    }

    // 获取总值（包含同步和未同步的部分）
    fn total(&self) -> u32 {
        let global = self.global_count.load(Ordering::Acquire);
        let local_sum: u32 = self
            .local_counts
            .iter()
            .map(|c| c.load(Ordering::Relaxed))
            .sum();
        global + local_sum
    }
}

fn delayed_counter_example() {
    println!("=== 延迟更新计数器示例 ===");

    let counter = Arc::new(DelayedCounter::new(4));
    let mut handles = vec![];

    for i in 0..4 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for j in 0..1000 {
                counter_clone.increment(i);

                // 每100次同步一次
                if j % 100 == 0 {
                    counter_clone.sync(i);
                }
            }

            // 最后一次同步
            counter_clone.sync(i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("总计数值: {}", counter.total());
    println!();
}

// ============================================
// 示例 3: 原子引用计数（类似 Arc 的内部实现）
// ============================================
struct AtomicRc<T> {
    data: T,
    ref_count: AtomicUsize,
}

impl<T> AtomicRc<T> {
    fn new(data: T) -> Self {
        AtomicRc {
            data,
            ref_count: AtomicUsize::new(1),
        }
    }

    fn clone(&self) -> Arc<Self> {
        let old_count = self.ref_count.fetch_add(1, Ordering::Relaxed);
        println!("引用计数增加: {} -> {}", old_count, old_count + 1);

        // 防止溢出
        if old_count > usize::MAX / 2 {
            panic!("引用计数溢出");
        }

        unsafe { Arc::from_raw(self as *const Self) }
    }

    unsafe fn drop_ref(&self) -> bool {
        let old_count = self.ref_count.fetch_sub(1, Ordering::Release);
        println!("引用计数减少: {} -> {}", old_count, old_count - 1);

        if old_count == 1 {
            self.ref_count.load(Ordering::Acquire);
            println!("最后一个引用被释放");
            return true;
        }
        false
    }
}

fn ref_count_example() {
    println!("=== 原子引用计数示例 ===");

    let created = Arc::new(AtomicRc::new(42));
    let mut handles = vec![];

    for i in 0..3 {
        let arc_clone = Arc::clone(&created);
        let handle = thread::spawn(move || {
            println!("线程 {}: 数据中值为 {}", i, arc_clone.data);
            thread::sleep(Duration::from_millis(100));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("最终引用计数: {}", created.ref_count.load(Ordering::SeqCst));
    println!();
}

// ============================================
// 示例 4: 交替打印（线程同步）
// ============================================
fn alternate_print_example() {
    println!("=== 交替打印示例 ===");

    let flag = Arc::new(AtomicBool::new(true));

    let flag_clone = Arc::clone(&flag);
    let handle1 = thread::spawn(move || {
        for i in 0..10 {
            // 等待 flag 为 true
            while !flag_clone.load(Ordering::Acquire) {}

            println!("线程 1: {}", i);

            // 设置 flag 为 false
            flag_clone.store(false, Ordering::Release);
        }
    });

    let flag_clone = Arc::clone(&flag);
    let handle2 = thread::spawn(move || {
        for i in 0..10 {
            // 等待 flag 为 false
            while flag_clone.load(Ordering::Acquire) {}

            println!("线程 2: {}", i);

            // 设置 flag 为 true
            flag_clone.store(true, Ordering::Release);
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
    println!();
}

// ============================================
// 示例 5: 无锁 Treiber 栈（简化版）
// ============================================
use std::ptr;

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: *const Node<T>,
}

#[derive(Debug)]
struct TreiberStack<T> {
    head: AtomicUsize, // 存储 Node 指针
    _marker: std::marker::PhantomData<T>,
}

unsafe impl<T: Send> Send for TreiberStack<T> {}
unsafe impl<T: Send> Sync for TreiberStack<T> {}

impl<T: std::fmt::Display> TreiberStack<T> {
    fn new() -> Self {
        TreiberStack {
            head: AtomicUsize::new(0),
            _marker: std::marker::PhantomData,
        }
    }

    fn push(&self, value: T) {
        unsafe {
            let new_node = Box::into_raw(Box::new(Node {
                value,
                next: ptr::null(),
            }));

            let mut current_head = self.head.load(Ordering::Acquire);

            loop {
                (*new_node).next = current_head as *const Node<T>;

                match self.head.compare_exchange_weak(
                    current_head,
                    new_node as usize,
                    Ordering::Release,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => {
                        println!("压入值: {}", (*new_node).value);
                        return;
                    }
                    Err(actual) => {
                        current_head = actual;
                    }
                }
            }
        }
    }
}

fn treiber_stack_example() {
    println!("=== Treiber 无锁栈示例 ===");

    let stack = Arc::new(TreiberStack::<i32>::new());
    let mut handles = vec![];

    for i in 0..3 {
        let stack_clone = Arc::clone(&stack);
        let handle = thread::spawn(move || {
            for j in 0..3 {
                stack_clone.push(i * 10 + j);
                thread::sleep(Duration::from_millis(1));
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("栈操作完成");
    println!();
}

fn main() {
    println!("高级原子操作示例\n");

    // 示例 1: 自旋锁
    spinlock_example();

    // 示例 2: 延迟更新计数器
    delayed_counter_example();

    // 示例 3: 引用计数
    ref_count_example();

    // 示例 4: 交替打印
    alternate_print_example();

    // 示例 5: Treiber 栈
    treiber_stack_example();

    println!("所有示例完成!");
}
