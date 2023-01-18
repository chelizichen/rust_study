struct Person{
  name:String,
  age:u8
}

struct Tuple_Test(u32,u16);

fn main(){
  let name = "123";
  let age:u8 = 12;
  let person:Person = Person{name:name.to_string(),age};
  println!("age is {}",person.age);
  println!("name is {}",person.name);

}