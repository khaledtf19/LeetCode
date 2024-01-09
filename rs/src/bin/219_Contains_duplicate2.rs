use std::{collections::HashMap, usize};

fn main() {
    let res = contains_nearby_duplicate(vec![-1, -2, -3, -1, -2, -3], 3);
    println!("{}", res);
}
pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut hm = HashMap::with_capacity(nums.len());
    for (i, num) in nums.iter().enumerate() {
        let v = hm.entry(num).or_insert(i);
        if i != *v && i.abs_diff(*v) as i32 <= k {
            return true;
        }
        if i != *v {
            *v = i
        }
    }
    false
}
