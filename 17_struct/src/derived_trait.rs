#[derive(Debug,Clone,Copy)]
pub struct Rectangle{
    pub width:i32,
    pub height:i32,
}
impl Rectangle{
    pub fn area(&self) -> i32{ // &self:&Self
        self.width * self.height
    }
    pub  fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
    pub  fn square(size:i32)->Self{ // 关联函数
        Self{
            width:size,
            height:size,
        }
    }
    //Rectangle::max(*self,other)
    pub fn max(self,other:Self)->Self{ // 如果没有Clone，会获得所有权
        let w=self.width.max(other.width);
        let h=self.height.max(other.height);
        Rectangle{
            width:w,
            height:h
        }
    }
    pub fn set_to_max(&mut self, other:Self){
        *self=self.max(other); // 没有Copy，会报错，不能移动
    }
}