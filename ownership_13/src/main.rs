fn main() {
    let mut v=vec![1,2,3]; 
    let num=&mut v[2]; // 失去了W O的权限
    v.push(1);  
    // println!("{}",num); 发生了借用
}