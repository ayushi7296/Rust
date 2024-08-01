// Ownership
// Ownership is a set of rules that govern how a rust program manages memory
// 1. Each Value in rust has a owner
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.


fn main() {
    println!("Ownership example");
    owner();
    ownership_in_primitive();
}
fn owner(){
    let vector1 = vec![1,2,3];
    let vector2 = vector1;
    println!("Value of vector2 {:?}", vector2);
    // println!("Value of vector1 {:?}", vector1); // It will throw the error
}
fn ownership_in_primitive(){
    let x= 20;
    println!("Value of x is {}", x); // It will pring "Value of x is 20"

    let y=x;
    println!("value of y is {}", y); // It will pring "Value of y is 20"

    println!("Printing x again {}",x );

    //In case of primitive data types, contents from one variable are copied to another. 
    //So, there is no ownership happening.

}
