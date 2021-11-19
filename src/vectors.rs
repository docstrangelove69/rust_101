// Vectors - Resizable arrays

pub fn run(){
    let mut numbers: Vec<i32> = vec![1,2,3,4];
    println!("vector is {:?}", numbers);
    //single element
    numbers[0] = 0;
    println!("updated vector is {:?}", numbers);

    //vector length
    numbers.push(5);
    println!(" vector length is {}", numbers.len());

    // pop of the last value
    let poped_value = numbers.pop();
    assert_eq!(Some(5), poped_value);
    println!(" vector popped value is {:?}", poped_value);
    println!("updated vector is {:?}", numbers);

    //loop over vector
    for (pos,e) in numbers.iter_mut().enumerate(){
        println!("index :{} val:{}",pos,e )
    }

    //loop and mutable values
    for x in numbers.iter_mut(){
        *x *=2;
    }

    println!("updated vector is {:?}", numbers);

}

