// Struct is a custom data type.
// A struct is a way to group together different fields with with different data types into a single unit,
// similar to a class in object-oriented programming. 

struct Shopping{            // Defining a struct
    apparel : String,
    size : i32,
    brand : String 
}

fn main() {
    println!("Struct Demo");

    let mut s1 = Shopping{          //initializing the struct
        apparel: String::from("TShirt"),
        size: 34,
        brand: String::from("Nike")
    };

    println!("Want to buy a  {}, of {} brand. And my size is {}", s1.apparel, s1.brand, s1.size);


    s1.size = 36; //example of mutable struct

    println!("My Tshirt's correct size is {}",s1.size);

}
