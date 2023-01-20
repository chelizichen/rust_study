
enum IpAddrKind {
    v4(String,u32),
    v6(String)
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn enum_test_fn(){
    let m1:IpAddr = IpAddr { 
        kind: IpAddrKind::v4(String::from("123"),456),
         address: String::from("123")
    };
    let m2:IpAddr = IpAddr { 
        kind: IpAddrKind::v6(String::from("456")), 
        address:String::from("123")
    };
}

#[derive(Debug)]
struct User{
    username:String,
    password:String,
    age:i32
}



impl User {
    fn getInfo(&self)->i32{
        self.age*5
    }
    fn is_big_than_30(&self)->bool{
        self.age > 30
    }

    fn which_is_old(&self,other:&User)->bool{
        return self.age>other.age;
    }
}

fn main() {
    let _vet:Vec<i32> = vec![1,2,3];
    let age = 32;
    let age1 = 32;


    let user = User{
        username:"123".to_string(),
        password:"456".to_string(),
        age
    };
    let user1 = User{
        username:"123".to_string(),
        password:"456".to_string(),
        age:age1
    };
    // let mut user1 = User{
    //     username:"456".to_string(),
    //     ..user
    // };
    user1.which_is_old(&user1);
    println!("Hello, world!");
    println!("{:#?}",user);
    // println!("{:#?}",user1);
    


}
