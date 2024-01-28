use std::collections::{HashMap, HashSet};

fn main() {
    let res = three_sum(vec![0, 0, 0]);
    println!("{:?}", res);
}

pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = HashSet::new();
    nums.sort_unstable();
    for (i, &num1) in nums.iter().enumerate() {
        let (mut l, mut r) = (0, i as i32 - 1);
        while l < r {
            let num2 = nums[l as usize];
            let num3 = nums[r as usize];
            match (num1 + num2 + num3).cmp(&0) {
                std::cmp::Ordering::Less => {
                    l += 1;
                }
                std::cmp::Ordering::Greater => {
                    r -= 1;
                }
                std::cmp::Ordering::Equal => {
                    let v = vec![num1, num3, num2];
                    res.insert(v);
                    while l < r && nums[l as usize] == nums[l as usize + 1] {
                        l += 1;
                    }
                    while l < r && nums[r as usize] == nums[r as usize - 1] {
                        r -= 1;
                    }
                    l += 1;
                    r -= 1;
                }
            }
        }
    }

    return res.into_iter().collect::<Vec<Vec<i32>>>();
}

pub fn three_sum_bad(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    for (i, num1) in nums.iter().enumerate() {
        let mut hm: HashMap<i32, Vec<usize>> = HashMap::new();
        for (j, num2) in nums.iter().enumerate() {
            if let Some(v) = hm.get(num2) {
                if !v.contains(&j) {
                    let mut new_res = vec![nums[v[0]], nums[v[1]], nums[j]];
                    new_res.sort();
                    if !res.contains(&new_res) {
                        res.push(new_res);
                    }
                }
            } else {
                if i != j {
                    let num = num1 + num2;
                    let new_v = vec![i, j];
                    if num > 0 {
                        hm.insert(num * -1, new_v);
                    } else if num <= 0 {
                        hm.insert(num.abs(), new_v);
                    }
                }
            }
        }
    }

    return res;
}
