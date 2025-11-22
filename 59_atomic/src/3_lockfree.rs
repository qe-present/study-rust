use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

// ============================================
// 示例 1: 基于 AtomicUsize 的计数器
// ============================================
struct SimpleCounter {
    count: AtomicUsize,
}

impl SimpleCounter {
    fn new() -> Self {
        SimpleCounter {
            count: AtomicUsize::new(0),
        }
    }

    fn increment(&self) -> usize {
        self.count.fetch_add(1, Ordering::SeqCst) + 1
    }

    fn get(&self) -> usize {
        self.count.load(Ordering::SeqCst)
    }

    fn decrement(&self) -> usize {
        self.count.fetch_sub(1, Ordering::SeqCst) - 1
    }
}

fn counter_example() {
    println!("=== 无锁计数器示例 ===\n");

    let counter = Arc::new(SimpleCounter::new());
    let mut handles = vec![];

    println!("初始值: {}", counter.get());

    for i in 0..5 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for j in 1..=5 {
                let value = counter_clone.increment();
                println!("线程 {}: {} -> {}", i, j, value);
                thread::sleep(Duration::from_millis(10));
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("\n最终值: {}", counter.get());
    // 测试递减
    println!("递减测试: {} -> {}", counter.get(), counter.decrement());
    println!();
}

// ============================================
// 示例 2: 无锁累加器
// ============================================
struct LockFreeAccumulator {
    value: AtomicUsize,
    min: AtomicUsize,
    max: AtomicUsize,
}

impl LockFreeAccumulator {
    fn new() -> Self {
        LockFreeAccumulator {
            value: AtomicUsize::new(0),
            min: AtomicUsize::new(usize::MAX),
            max: AtomicUsize::new(0),
        }
    }

    fn add(&self, n: usize) {
        let _current = self.value.fetch_add(n, Ordering::SeqCst);

        loop {
            let min_val = self.min.load(Ordering::SeqCst);
            if n < min_val {
                if self.min.compare_exchange(min_val, n, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                    break;
                }
            } else {
                break;
            }
        }

        loop {
            let max_val = self.max.load(Ordering::SeqCst);
            if n > max_val {
                if self.max.compare_exchange(max_val, n, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                    break;
                }
            } else {
                break;
            }
        }
    }

    fn get_sum(&self) -> usize {
        self.value.load(Ordering::SeqCst)
    }

    fn get_average(&self, count: usize) -> f64 {
        if count == 0 {
            0.0
        } else {
            self.get_sum() as f64 / count as f64
        }
    }

    fn get_min(&self) -> usize {
        self.min.load(Ordering::SeqCst)
    }

    fn get_max(&self) -> usize {
        self.max.load(Ordering::SeqCst)
    }

}

fn accumulator_example() {
    println!("=== 无锁累加器示例 ===\n");

    let acc = Arc::new(LockFreeAccumulator::new());
    let mut handles = vec![];

    let values = vec![3, 7, 2, 9, 5, 1, 8, 4, 6, 10];

    for (i, &value) in values.iter().enumerate() {
        let acc_clone = Arc::clone(&acc);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis((i * 5) as u64));
            acc_clone.add(value);
            println!("添加值: {}", value);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("\n总计: {}", acc.get_sum());
    println!("最小值: {}", acc.get_min());
    println!("最大值: {}", acc.get_max());
    println!("平均值: {:.1}", acc.get_average(values.len()));
    println!();
}

// ============================================
// 示例 3: 无锁进度追踪器
// ============================================
#[derive(Debug)]
struct ProgressTracker {
    total: usize,
    completed: AtomicUsize,
    failed: AtomicUsize,
}

impl ProgressTracker {
    fn new(total: usize) -> Self {
        ProgressTracker {
            total,
            completed: AtomicUsize::new(0),
            failed: AtomicUsize::new(0),
        }
    }

    fn complete_one(&self) {
        self.completed.fetch_add(1, Ordering::SeqCst);
    }

    fn fail_one(&self) {
        self.failed.fetch_add(1, Ordering::SeqCst);
    }

    fn get_progress(&self) -> (usize, usize, f64) {
        let completed = self.completed.load(Ordering::SeqCst);
        let failed = self.failed.load(Ordering::SeqCst);
        let progress = (completed as f64 / self.total as f64) * 100.0;
        (completed, failed, progress)
    }

}

fn progress_tracker_example() {
    println!("=== 无锁进度追踪器示例 ===\n");

    let tracker = Arc::new(ProgressTracker::new(20));
    let mut handles = vec![];
    use rand::Rng;

    for i in 0..5 {
        let tracker_clone = Arc::clone(&tracker);
        let handle = thread::spawn(move || {
            let mut rng = rand::rng();
            for _ in 0..4 {
                thread::sleep(Duration::from_millis(20));
                if rng.random_bool(0.8) {
                    tracker_clone.complete_one();
                } else {
                    tracker_clone.fail_one();
                }
                let (completed, failed, progress) = tracker_clone.get_progress();
                println!("线程 {}: 已完成 {}, 失败 {}, 进度 {:.1}%", i, completed, failed, progress);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let (completed, failed, progress) = tracker.get_progress();
    println!("\n最终结果:");
    println!("已完成: {}", completed);
    println!("失败: {}", failed);
    println!("成功率: {:.1}%", progress);
    println!();
}

// ============================================
// 示例 4: 无锁任务调度器
// ============================================
#[derive(Clone, Copy)]
struct Task {
    id: usize,
    priority: usize,
}

impl Task {
    fn new(id: usize, priority: usize) -> Self {
        Task { id, priority }
    }
}

struct TaskScheduler {
    tasks: Vec<Arc<crossbeam::atomic::AtomicCell<Option<Task>>>>,
    size: usize,
}

impl TaskScheduler {
    fn new(size: usize) -> Self {
        let mut tasks = Vec::with_capacity(size);
        for _ in 0..size {
            tasks.push(Arc::new(crossbeam::atomic::AtomicCell::new(None)));
        }
        TaskScheduler { tasks, size }
    }

    fn submit(&self, task: Task) -> bool {
        for slot in &self.tasks {
            if slot.swap(Some(task.clone())).is_none() {
                return true;
            }
        }
        false
    }

    fn execute(&self) -> Option<Task> {
        for slot in &self.tasks {
            if let Some(task) = slot.swap(None) {
                println!("执行任务: ID={}, 优先级={}", task.id, task.priority);
                return Some(task);
            }
        }
        None
    }

    fn count(&self) -> usize {
        self.tasks.iter().filter(|slot| slot.load().is_some()).count()
    }
}

fn task_scheduler_example() {
    println!("=== 无锁任务调度器示例 ===\n");

    let scheduler = Arc::new(TaskScheduler::new(10));
    let mut handles = vec![];

    for i in 0..3 {
        let scheduler_clone = Arc::clone(&scheduler);
        let handle = thread::spawn(move || {
            for j in 0..5 {
                let task = Task::new(i * 10 + j, (i * 10 + j) % 5);
                if scheduler_clone.submit(task) {
                    println!("生产者 {}: 提交任务 {}-{}", i, i, j);
                } else {
                    println!("生产者 {}: 队列满, 无法提交任务 {}-{}", i, i, j);
                }
                thread::sleep(Duration::from_millis(5));
            }
        });
        handles.push(handle);
    }

    thread::sleep(Duration::from_millis(30));

    let executor_handle = {
        let scheduler_clone = Arc::clone(&scheduler);
        thread::spawn(move || {
            for _ in 0..20 {
                if let Some(_task) = scheduler_clone.execute() {
                    thread::sleep(Duration::from_millis(10));
                } else {
                    break;
                }
            }
        })
    };

    for handle in handles {
        handle.join().unwrap();
    }
    executor_handle.join().unwrap();

    println!("队列剩余任务: {}\n", scheduler.count());
}

// ============================================
// 示例 5: 无锁状态管理器
// ============================================
struct StateManager {
    counter: AtomicUsize,
    operation_count: AtomicUsize,
    last_operation: AtomicUsize, // 0=start, 1=stop, 2=reset
}

impl StateManager {
    fn new() -> Self {
        StateManager {
            counter: AtomicUsize::new(0),
            operation_count: AtomicUsize::new(0),
            last_operation: AtomicUsize::new(0),
        }
    }

    fn start(&self) {
        let current = self.counter.fetch_add(1, Ordering::SeqCst);
        self.operation_count.fetch_add(1, Ordering::SeqCst);
        self.last_operation.store(0, Ordering::SeqCst);
        println!("启动: 计数={}", current + 1);
    }

    fn stop(&self) {
        let current = self.counter.fetch_sub(1, Ordering::SeqCst);
        self.operation_count.fetch_add(1, Ordering::SeqCst);
        self.last_operation.store(1, Ordering::SeqCst);
        println!("停止: 计数={}", current.saturating_sub(1));
    }

    fn reset(&self) {
        let old = self.counter.swap(0, Ordering::SeqCst);
        self.operation_count.fetch_add(1, Ordering::SeqCst);
        self.last_operation.store(2, Ordering::SeqCst);
        println!("重置: 旧值={}", old);
    }

    fn get_stats(&self) -> (usize, usize, usize) {
        let count = self.counter.load(Ordering::SeqCst);
        let operations = self.operation_count.load(Ordering::SeqCst);
        let last_op = self.last_operation.load(Ordering::SeqCst);
        (count, operations, last_op)
    }
}

fn state_manager_example() {
    println!("=== 无锁状态管理器示例 ===\n");

    let manager = Arc::new(StateManager::new());
    let mut handles = vec![];

    for _i in 0..3 {
        let manager_clone = Arc::clone(&manager);
        let handle = thread::spawn(move || {
            for _ in 0..5 {
                manager_clone.start();
                thread::sleep(Duration::from_millis(5));
                manager_clone.stop();
            }
        });
        handles.push(handle);
    }

    thread::sleep(Duration::from_millis(50));

    let reset_handle = {
        let manager_clone = Arc::clone(&manager);
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(20));
            manager_clone.reset();
        })
    };

    for handle in handles {
        handle.join().unwrap();
    }
    reset_handle.join().unwrap();

    let (count, operations, last_op) = manager.get_stats();
    let last_op_str = match last_op {
        0 => "start",
        1 => "stop",
        2 => "reset",
        _ => "unknown",
    };
    println!("\n最终统计:");
    println!("当前计数: {}", count);
    println!("操作次数: {}", operations);
    println!("最后操作: {}", last_op_str);
    println!();
}

fn main() {
    println!("无锁（Lock-Free）同步示例\n");
    println!("=======================================\n");

    counter_example();
    thread::sleep(Duration::from_millis(100));

    accumulator_example();
    thread::sleep(Duration::from_millis(100));

    progress_tracker_example();
    thread::sleep(Duration::from_millis(100));

    task_scheduler_example();
    thread::sleep(Duration::from_millis(100));

    state_manager_example();

    println!("所有无锁示例完成!");
}
