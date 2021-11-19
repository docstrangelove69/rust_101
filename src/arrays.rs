// Arrays - Fixed list where elements are the same data types
use std::mem;

pub fn run()
{
    let mut number:[i32;5] = [1,2,3,4,5];
    println!("{:?}", number);

    //get single val
    println!("index 0 : {}", number[0]);

    //reassign
    number[1] = 12;
    println!("index 1 : {}", number[1]);

    //length
    println!("length of array {}", number.len());

    //Arrays of stack allocated
    println!("Array occupies {} Bytes", mem::size_of_val(&number));

    //get slice
    let slice: &[i32] = &number[0..2];
    println!("Slice : {:?}", slice);


}