fn main() {
    println!("Option demo");
    calculate(20);
    calculate(21);
}

fn calculate(no:i32) -> Option<bool> 
{
    if no%2 == 0{
        Some(true)
    } else {
        None
    }

}