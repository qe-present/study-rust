use object::{Button, Draw, Screen};
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}
impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing a select box with width: {}, height: {}, options: {:?}", self.width, self.height, self.options);
    }
}
fn main() {
    let screen= Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 20,
                label: String::from("OK"),
            }),
            // Box::new(String::from("Hello, world!"))
            // The trait bound `String: Draw` is not satisfied
        ],
    };
    screen.run();
}