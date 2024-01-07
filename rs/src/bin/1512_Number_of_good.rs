fn main() {
    let res = num_identical_pairs(vec![1, 2, 3, 1, 1, 3]);
    println!("{res}");
}

pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut pairs = 0;
    for (i, num) in nums.iter().enumerate() {
        pairs += nums.iter().skip(i + 1).filter(|n| *n == num).count();
    }

    return pairs as i32;
}
