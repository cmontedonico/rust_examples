//Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run(){
    let name = "Brad";
    let mut age = 23;

    age = 38;

    println!("My name is {} and i am  {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID {}", ID);

    //Multiple variables at once
    let ( my_name, my_age ) = ("Cesare", 37);
    println!("{} is {}", my_name, my_age);
}