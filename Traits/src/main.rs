//Traits
// Traits tell compiler about the functionality a particular type has and can share with other types.
// Traits are an abstract behaviour amongst different data types.
pub trait Maths { 
    fn area(&self) -> i32; 
    fn perimeter(&self) ->i32; 
}
struct Parameter{
    length: i32, 
    breadth: i32}
impl Maths for Parameter{
    fn area(&self) -> i32 { 
        return self.length * self.breadth; 
    }
 fn perimeter(&self) ->i32 {
    return 2*(self.length + self.breadth); 
    }
}

fn main() { 
    println!("Traits Example");
    let para = Parameter{ 
        length: 10, 
        breadth: 12 
    };
    println!("Area of the Rectangle {}" , para.area()); 
    println!("Perimeter of the Rectangle {} ", para.perimeter());
}