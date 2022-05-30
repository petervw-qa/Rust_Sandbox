// Variables hold primitive data or references to data
// Variables are immutable by defauly
// Rust is a block-scoped language


pub fn run() {
    let name = "Peter";
    let mut age = 24;

    println!("My name is {}! I am {} :)", name, age);

    age = 25;

    println!("My name is {}! I am not {} :)", name, age);

    // Define Constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple variables
    let ( my_name, my_age ) = ("Peter", 24);
    println!("{} is {}", my_name, my_age);


}