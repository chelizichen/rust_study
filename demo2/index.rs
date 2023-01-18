use std::mem;

fn analyze_slice(slice: &[i32]){
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main(){
  let xs:[i32;5] = [1,2,3,4,5];
  // let ys:[u32;100] = [0;100]
  println!("first element of the array: {}", xs[0]);
  println!("second element of the array: {}", xs[1]);
  println!("array occupies {} bytes", mem::size_of_val(&xs)); // 12 字节
  analyze_slice(&xs[1 .. 4]);
}