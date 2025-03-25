/*
HashMap<K,V>
创建
    let hm=HashMap::new();
    hm.insert(String::from("a"), 1);
    hm.insert(String::from("b"), 2);

        let v=vec![(1,"hello"),(2,"world")];
    let map:HashMap<_,_> =v.into_iter().collect();

        let mp=HashMap::from([
        ("key1",1),
        ("key2",2),
        ("key3",3)
        ]
    );
K类型相同
V类型相同
get方法取值
    let mp=HashMap::from([
        ("key1",1),
        ("key2",2),
        ("key3",3)
        ]
    );
    let a=mp.get(&"key1").copied().unwrap_or(0);
    println!("key1 = {}", a);

 遍历
     for (k,v) in &mp{
        println!("{},{}",k,v);

    }
所有权
1、具有copy trait的值，复制到map
2、具有所有权的值，移动到map里面

更新
当有key存在，插入insert时，替换掉旧值
    let mut mp=HashMap::from([
        ("key1",1),
        ("key2",2),
        ("key3",3)
        ]
    );
    mp.insert("key1", 11);
    println!("{mp:?}")
    
   
    mp.entry("key1").or_insert(11); // entry 判断存在还是不存在，or_insert不存在，把值插入
    
        let text="Hello world hello ni hao world"; // 统计单词出现的次数
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count=map.entry(word).or_insert(0);
        *count+=1;
    }
    println!("{:?}", map);

 */

use std::collections::HashMap;

fn main() {
}
// #[test]
// fn test1() {
//     let mut h=HashMap::new();
//     h.insert("a", 1);
//     let v1=&h["a"];
//     h.insert("b", 2); // 发生了可变的借用
//     let v2=&h["b"];
//     println!("{},{}",v1,v2);
//     
//     
// }
