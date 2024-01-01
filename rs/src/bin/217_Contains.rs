use std::collections::HashSet;

fn main() {
    let res = contains_duplicate(vec![1, 2, 3]);
    println!("{}", res);
}

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut hs: HashSet<i32> = HashSet::with_capacity(nums.len());
    for num in nums.iter() {
        let bol = hs.insert(*num);
        match bol {
            true => {}
            false => return true,
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_true() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 1]), true)
    }

    #[test]
    fn test_contains_false() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 4]), false)
    }
}
