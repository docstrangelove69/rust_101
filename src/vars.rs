//Variables hold primitive data or references to data
//Variables are immutable by default
//Rust is block scoped language

pub fn run()
{
    let name = "Elon";
    let mut age = "50";
    age = "52";
    println!("My name is {} and my age is {}", name, age);

    //Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Multiple variables
    let (my_name, my_age )= ("Elon", 50);
    println!("my name is {} and my age is {} ", my_name, my_age);

    //multiple variable with mut
    let  (mut my_othername, mut my_otherage )= ("Elonother", 53);
    println!("my name is {} and my age is {} ", my_othername, my_otherage);
    my_othername="ELonModified";
    my_otherage=54;
    println!("my name is {} and my age is {} ", my_othername, my_otherage);

}