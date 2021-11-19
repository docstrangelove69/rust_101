pub fn run()
{
    //Print to console
    println!("Hello from print.rs file");

    //Basic Formatting
    println!("Number {}",1);
    println!{"{} is from {}", "Elon", "Mars" };

    //Positional Arguments
    println!("{0} is from {1} and {0} like to do {2}","Elon","Mars","code");

    //Named Arguments
    println!("{name} likes to play {activity} ", name="Elon", activity="football");

    //placeholder traits
    println!("Binary : {:b} Hex : {:x} Octal : {:o}", 10, 10, 10); 

    //placeholder for debug trait
    println!("{:?}",(12, true, "hello"));

    //Basic math
    println!("{}",10+10);
}