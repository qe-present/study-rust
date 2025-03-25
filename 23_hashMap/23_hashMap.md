# HashMap_23

哈希映射（HashMap）相关学习笔记。

## 学习总结

1. `HashMap<K, V>` 储存键值对的映射关系
2. 需要手动导入：`use std::collections::HashMap;`
3. 创建新的HashMap：`let mut scores = HashMap::new();`
4. 使用键值对插入数据：`scores.insert(String::from("Blue"), 10);`
5. 从vector创建HashMap：使用`collect`方法
6. 所有键必须是相同类型，所有值也必须是相同类型
7. 访问值：`scores.get(&String::from("Blue"))` 返回 `Option<&V>`
8. 遍历：`for (key, value) in &scores { ... }`
9. 更新HashMap的几种方式：
   - 覆盖已有的值：`scores.insert(key, value)`
   - 只在键没有对应值时插入：`scores.entry(key).or_insert(value)`
   - 基于旧值更新：获取可变引用后修改
10. HashMap默认使用SipHash哈希函数，安全但较慢
11. 可以使用不同的hasher来优化性能

---

| [上一页：字符串](../22_string/22_string.md) | [下一页：错误处理](../24_error/24_error.md) |
|------------------------|------------------------| 