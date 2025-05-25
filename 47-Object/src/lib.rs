/// trait object
/// 1 引用或者指针+dyn+trait=> trait object
pub trait Draw {
    fn draw(&self);
}
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
impl Draw for Button {
    fn draw(&self) {
        println!("Drawing a button with width: {}, height: {}, label: {}", self.width, self.height, self.label);
    }
}
// #[cfg(test)]
// mod tests {
//     use std::fmt::Debug;
//     use super::*;
//     #[test]
//     fn test_draw() {
//         let n=1;
//         let s=String::from("hello");
//         let v:Vec<&dyn Debug>= vec![&n, &s];
//         let n_ref=v[0] as &i32; //Non-primitive cast: `&dyn Debug` as `&i32`
//         println!("{}",n_ref);
//     }
// }