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