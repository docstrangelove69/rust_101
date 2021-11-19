
// Loops - Used to iterate until a condition is met
pub fn run()
{
    let mut count = 0;

    //infinite loop
    // loop {
    //     count +=1;
    //     println!("Number {}", count);

    //     if count ==20{
    //         break;
    //     }
    // }

    //while loop
    // while count<20{
    //     if count%15 ==0{
    //         println!("FIZZBUZZ");
    //     }else if count%3 == 0{
    //         println!("FIZZ");
    //     }else if count%5 ==0{
    //         println!("BUZZ");
    //     }else{
    //         println!("{}", count);
    //     }
    //     count +=1;
    // }

    // for range loop
    for x in count..20{
        if x%15 ==0{
            println!("FIZZBUZZ");
        }else if x%3 == 0{
            println!("FIZZ");
        }else if x%5 ==0{
            println!("BUZZ");
        }else{
            println!("{}", x);
        }
    }
}