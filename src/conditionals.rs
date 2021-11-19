// Conditionals - Used to check the condition of something and act on the result

pub fn run()
{
    let age = 22;
    let check_id = false;
    let knows_persons_age = true;

    if age>=21 && check_id || knows_persons_age{
        println!("Bartender : what drink you want?");
    }
    else if age>= 21{
        println!("Bartender : I Need to see your ID");
    }
    else{
        println!("Bartender: sorry, you have to leave!");
    }

    //short hand if
    let is_of_age = if age >=21{true}else{false};
    println!("is_of_age :{}",is_of_age);

}