// A reference represents a borrow of some owned value.
// & is used for referencing while * is used for dereferencing.
// Action of creating a reference is called borrowing.



fn main() {
    println!("References");
    let x = 10;
    let y = &x; // creating reference of x.

    println!("Value of y {}", y);

    let mut a = 100;
    let b = &mut a; // creating mutable reference
    println!("value of b {}", b);

    *b += 10; // Changing value of a.

    println!("Value of a {}", a);

}
