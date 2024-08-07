#[derive(Debug)]
enum Gender {               //enum declared
    Male,
    Female,
    Trans
}
#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
    gender : Gender         // use of enum inside a struct
}
fn main() {
    println!("Enum Advance topics");

    let Person1 = Person{
        name:String::from("Ayushi"),
        age: 26,
        gender:Gender::Female
    };

    println!("Details are : {:?}", Person1);
}

