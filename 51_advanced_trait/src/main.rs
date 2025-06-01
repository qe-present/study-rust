/// 关联类型
/// - 关联类型将一个“类型占位符”绑定到trait中
/// - 允许trait的方法使用该类型占位符
/// - 实现trait时指定具体类型
/// 默认泛型类型参数
/// 当使用泛型类型参数时，可以为其提供默认类型
/// - 使用`<T = DefaultType>`语法
/// - 如果实现时未指定类型，则使用默认类型
use std::ops::Add;
use std::fmt::Display;


/// trait Add<RHS = Self> {
///    type Output;
///    fn add(self, rhs: RHS) -> Self::Output;
/// }


struct Point{
    x: i32,
    y: i32,
}
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
/// 完全限定语法
///  - Rust允许多个trait 实现相同的方法
///  - 使用完全限定语法来指定调用哪个trait的方法
/// - 语法为 `TraitName::method_name(self, args...)
trait Animal{
    fn baby_name() -> String;
}
struct Dog;
impl Animal for Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}
impl Dog{
    fn baby_name() -> String {
        String::from("Fido")
    }
}
/// super trait
/// - Rust允许trait继承其他trait
/// - 使用`super trait`可以在一个trait中使用另一个trait的方法
trait OutlinePrint{
    fn outline_print(&self);
}
impl <T: Display> OutlinePrint for T {
    fn outline_print(&self) {
        let output = format!("{}", self);
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("* {} *", output);
        println!("{}", "*".repeat(len + 4));
    }
}

struct NewPoint {
    x: i32,
    y: i32,
}
impl Display for NewPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NewPoint({}, {})", self.x, self.y)
    }
}
/// newtype模式
/// - Rust允许使用新类型模式来创建新的类型
/// - 新类型模式是通过创建一个包含单个字段的结构体来实现的
/// 孤儿规则 
/// - Rust的孤儿规则允许在没有冲突的情况下实现trait
/// - 如果trait没有其他实现，并且类型没有其他实现，则可以实现该trait
struct Wrapper(Vec<String>);
impl Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}]", self.0.join(", "))
    }
}
fn main(){
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2; // 使用 Add trait
    println!("Point p3: ({}, {})", p3.x, p3.y);
    let dog_baby_name = Dog::baby_name(); // 调用  dog 的方法
    println!("Dog baby name ({})", dog_baby_name);
    let dog_baby_name = <Dog as Animal>::baby_name(); // 调用 Animal 的方法
    println!("Dog baby name ({})", dog_baby_name);
    let point = NewPoint { x: 5, y: 6 };
    point.outline_print(); // 使用 super trait 的方法
    let wrapper = Wrapper(vec![String::from("Hello"), String::from("World")]);
    wrapper.outline_print(); // 使用 super trait 的方法
    
}
