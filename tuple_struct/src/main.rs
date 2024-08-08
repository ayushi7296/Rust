fn main() {
    println!("Tuple in struct");

    struct User (u8, u8, u8);

    let mut user1 = User(5,90,34);
    println!("user 1 values are : {} , {} , {}", user1.0,user1.1,user1.2);

    user1.1=23;

    println!("user1 values after alteration are : {} , {} , {}", user1.0,user1.1,user1.2);

    struct UserDetails (String, i16,bool);

    let user2 = UserDetails(String::from("Ayushi"), 12, true);

    println!("user values are : {} , {} , {}", user2.0,user2.1,user2.2);



}
