/*
    * Slice 返回 &str
    String slice [起始位置..结束位置] 左闭右开
    17 |     let word=first_word(&s);
   |                         -- immutable borrow occurs here
18 |     s.clear();
   |     ^^^^^^^^^ mutable borrow occurs here
19 |     println!("{}",word);
   |                   ---- immutable borrow later used here


 */
fn main() {
    let mut s=String::from("hello");
    let slice=&s[0..2]; // 0到2,左闭右开
    println!("{}",slice);
    let slice=&s[0..=2]; // 0到2 包含2
    println!("{}",slice);
    let slice=&s[..2]; // 0到2 包含2
    println!("{}",slice);
    let slice=&s[2..]; // 2到最后
    println!("{}",slice);
    let slice=&s[..]; // 全部
    println!("{}",slice);
    let word=first_word(&s);
    s.clear();
    println!("{}",word);
}

fn first_word(s:&str)->&str {
    let bytes=s.as_bytes();
    for (i,&item) in bytes.iter().enumerate(){
        if(item==b' '){
            return &s[..i];
        }
    }
    &s[..]

}