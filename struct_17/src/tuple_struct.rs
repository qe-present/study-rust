pub struct Color(i32,i32,i32);
impl Color {
    pub fn new(r:i32,g:i32,b:i32)->Color{
        Color(r,g,b)
    }
}