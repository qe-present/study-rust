use std::io::stdin;
// Prelude
fn main(){
    let mut msg= String::new();
    println!("Please Enter your name:");
    stdin().read_line(&mut msg).unwrap();
    println!("your name is {}", msg);

}
// rustc hello_world.rs
// crate
// -- library crate(1)
// -- binary crate(n)