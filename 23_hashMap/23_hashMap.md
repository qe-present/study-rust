# HashMap_23

哈希映射相关学习笔记。

## 学习总结

1. `HashMap<K, V>`存储键值对，通过键快速查找值
2. 不像Vector和String，HashMap不在预导入模块中
3. 创建HashMap：`let mut scores = HashMap::new();`
4. 插入键值对：`scores.insert(String::from("Blue"), 10);`
5. 从两个向量创建HashMap：`HashMap::from_iter(zip(keys, values))`
6. 访问值：`scores.get(&key)`返回`Option<&V>`
7. 遍历键值对：`for (key, value) in &scores { ... }`
8. 更新HashMap的方式：
   - 覆盖已有值：`insert`会替换现有值
   - 只在键不存在时插入：`entry().or_insert()`
   - 基于旧值更新：`*entry.or_insert(0) += 1;`
9. HashMap默认使用SipHash哈希函数，安全但较慢
10. 可使用其他实现`BuildHasher`特性的哈希函数提高性能
11. 与其他集合一样，当HashMap被丢弃时，所有键值对同时被丢弃 