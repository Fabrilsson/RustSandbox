//Variables hold primitive data or references to data
//Variables are immutable by default
//Rust is a block-scoped language

pub fn run(){
    let name = "Fabrício"; //immutable
    let mut age = 24; //mutable

    age = 25;

    println!("My name is {} and I am {}", name, age);

    //Define constants
    const id: u8 = 001;
    println!("ID: {}", id);

    //Assign multiple vars
    let (myName, myAge) = ("Fabrício", 24);
    println!("{} is {}", myName, myAge);
}