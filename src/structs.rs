
//traditional struct
struct Color{
    red :u8,
    green :u8,
    blue : u8
}

 //Tuple struct
 struct Tuplecolor(u8,u8,u8);

 //struct implementation with fn
 struct Person{
     first_name: String,
     last_name:String
 }

 impl Person{
     //construct person
     fn new(first:&str, last:&str)->Person{
         Person{
             first_name : first.to_string(),
             last_name : last.to_string()
         }
     }
     //Get full name
     fn full_name(&self)-> String{
        format!("{} {}", self.first_name, self.last_name)
     }

     //set last name
     fn set_lat_name(&mut self, last:&str){
         self.last_name = last.to_string();
     }

     //to tuple
     fn to_tuple(self)->(String, String){
         (self.first_name, self.last_name)
     }
 } 

pub fn run(){
   let mut c = Color{
       red:255,
       green:0,
       blue:0
   };
   println!("color {} {} {}", c.red, c.green, c.blue);
   c.red = 200;
   println!("color {} {} {}", c.red, c.green, c.blue);

  let mut d = Tuplecolor(255,0,0);
  println!("Tuplecolor {} {} {}", d.0, d.1, d.2);
  d.0=200;
  println!("Tuplecolor {} {} {}", d.0, d.1, d.2);

  let mut p = Person::new("Elon", "Musk");
  println!("first : {} last : {}", p.first_name, p.last_name);

  //get full name 
  println!("Persnon fullname : {}",p.full_name());

  p.set_lat_name("Polar");
  println!("updated first : {} last : {}", p.first_name, p.last_name);

  println!("tuple values {:?}", p.to_tuple())

}