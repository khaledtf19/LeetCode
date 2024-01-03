use std::{collections::HashMap, i32};

fn main() {
    let res = two_sum(vec![2, 7, 11, 5], 9);
    println!("{res:?}");
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hm: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
    for (i, num) in nums.iter().enumerate() {
        match hm.get(&num) {
            Some(res) => {
                return vec![*res, i as i32];
            }
            None => {
                hm.insert(target - num, i as i32);
            }
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2, 7, 11, 5], 9), vec![0, 1]);
    }
}
