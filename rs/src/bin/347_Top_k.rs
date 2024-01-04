use std::collections::HashMap;

fn main() {
    let res = top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
    println!("{:?}", res);
}

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut hm: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
    for num in nums.iter() {
        match hm.get_mut(num) {
            Some(v) => *v += 1,
            None => {
                hm.insert(*num, 1);
            }
        }
    }
    let mut res = hm.into_iter().collect::<Vec<(i32, i32)>>();
    res.sort_by(|a, b| b.1.cmp(&a.1));
    return res
        .drain(0..k as usize)
        .map(|(key, _)| key)
        .collect::<Vec<i32>>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_k_frequent() {
        assert_eq!(top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2])
    }
}
