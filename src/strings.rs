// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run()
{

// Primitive str
let hello = "hello";
println!("{}", hello);

//String = Growable, 
let mut text = String::from("Hello, this is Elon");
println!("{}", text);

//get length
println!("length of hello {}", hello.len());
println!("length of text {}", text.len());

text.push('h');
println!("length of text {}", text.len());

text.push_str("musk");
println!("length of text {}", text.len());

//capacity (no of bytes it can store)
println!("capacity of text {}", text.capacity());

//isempty
println!("is text is expmpty {}", text.is_empty());

//contains
println!("is text contains Elon {}", text.contains("Elon"));

//Replace
text = text.replace("musk", "polar");
println!(" the new wors is {}", text);

//Loop through strings by whitespace

for word in text.split_whitespace(){
    println!("{}", word);
}

//Create with certain capacity
let mut strf = String::with_capacity(10);
strf.push_str("hithisissi");
println!("strf is {}", strf);

//push extra character
strf.push('a');
println!("new strf is {}", strf);

//Assertion testing
assert_eq!(11,strf.len());
assert_eq!(10,strf.capacity());

}