# Rust Study

## 二分查询

````Rust
fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut start: i32 = 0;
    let mut endl = nums.len() as i32;
    let mut index: i32 = -1;
    loop{
        let half = (start + endl) / 2;
        let curr_half = nums[half as usize];
        if start + 1 == endl {
            if curr_half == target {
                index = start;
                break;
            } else {
                break;
            }
        };
        if  curr_half == target {
            index = half;
            break;
        };
        if  curr_half > target {
            endl = half;
        } else {
            start = half;
        };
    }
    return index;
}
````

## 枚举 和 结构体的使用
````rust
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
````


## 枚举和方法的使用

````RUST
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
````