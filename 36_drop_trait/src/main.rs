#[derive(Debug)]
struct Point{
    data:i32
}
impl Drop for Point{
    fn drop(&mut self) {
        println!("Point {} is dropped",self.data);
    }
}
fn main() {
    let p=Point{data:1};
    let p1=Point{data:2};
    drop(p);
}