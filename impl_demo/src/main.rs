fn main() {
    println!("Hello, world!");
    let my_rect = Rectangle{height:20, width: 30};
    println!("Area of rec is {} ", my_rect.area());
    println!("Area of rec is {} ", my_rect.is_square());

}
struct Rectangle {height:i32, width: i32}

impl Rectangle{
    fn area (&self) -> i32 {
        self.height * self.width
    }

    fn is_square(&self) -> bool {
        self.height==self.width       
    }

}