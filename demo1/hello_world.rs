struct Person<'a> {
    name: &'a str,
    age: u8,
}
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    println!("hello world");
    println!("hello i am chelizichen");
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    println!("{}",peter.name);
    // println!("x is x = {}", x)
    println!("{}",peter.age);
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix)

    

}
