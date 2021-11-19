
/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

// Rust is a statically typed language, which means that it must know the types of all variables at compile time, however, the compiler can usually infer what type we want to use based on the value and how we use it.

pub fn run()
{
    //by default i32
    let x = 1;
    //by default f64
    let y = 2.5;

    //Add explicit type
    let i:i64 = 1209387378579874;

    //find Max size
    println!("Max i32: {}", std::i32::MAX);
    //println!("Max f64: {}", std::f64::MAX);

    //Boolean 
    let active = true;

    //boolean from expression
    let is_x_greatethan_y = 1 > 3;

    //char
    let character ='a';

    let smiling_face ='\u{1F600}';


    println!("{:?}", (x,y,i,active, is_x_greatethan_y, character, smiling_face));
}
