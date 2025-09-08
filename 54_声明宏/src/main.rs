use declare_macros::my_vec;
fn main() {
    let v = my_vec!(2; 4);
    let v1=my_vec!(1,2,);
    println!("{:?}", v);
    println!("{:?}", v1);
}